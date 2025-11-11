use super::super::plugin::*;

use {
    std::collections::*,
    waki::{header::*, *},
};

const DEFAULT_MAX_REDIRECTIONS: usize = 10;

//
// HttpClient
//

/// HTTP client.
pub struct HttpClient {
    client: Client,

    /// Headers to use for all requests.
    pub headers: Vec<(HeaderName, HeaderValue)>,

    /// Max redirections. Set to 0 to disable redirections.
    pub max_redirections: usize,
}

/// Headers.
pub type Headers = Vec<(String, String)>;

impl Default for HttpClient {
    fn default() -> Self {
        Self { client: Client::default(), headers: Default::default(), max_redirections: DEFAULT_MAX_REDIRECTIONS }
    }
}

impl HttpClient {
    /// Add header to use for all requests.
    pub fn add_header<NameT, ValueT>(&mut self, name: NameT, value: ValueT) -> Result<(), DispatchError>
    where
        NameT: AsRef<str>,
        ValueT: AsRef<str>,
    {
        add_header(&mut self.headers, name, value)
    }

    /// Get string.
    pub fn get_string<HeadersT, NameT, ValueT>(&self, url: &str, headers: HeadersT) -> Result<String, DispatchError>
    where
        HeadersT: IntoIterator<Item = (NameT, ValueT)>,
        NameT: AsRef<str>,
        ValueT: AsRef<str>,
    {
        let body = self.get_bytes(url, headers)?;
        let body = String::from_utf8(body).map_err(|error| error.to_string())?;
        Ok(body)
    }

    /// Get bytes.
    pub fn get_bytes<HeadersT, NameT, ValueT>(&self, url: &str, headers: HeadersT) -> Result<Vec<u8>, DispatchError>
    where
        HeadersT: IntoIterator<Item = (NameT, ValueT)>,
        NameT: AsRef<str>,
        ValueT: AsRef<str>,
    {
        let response = self.request(Method::Get, url, headers)?;
        let body = response.body().map_err(|error| error.to_string())?;
        Ok(body)
    }

    /// Request.
    pub fn request<HeadersT, NameT, ValueT>(
        &self,
        method: Method,
        url: &str,
        headers: HeadersT,
    ) -> Result<Response, DispatchError>
    where
        HeadersT: IntoIterator<Item = (NameT, ValueT)>,
        NameT: AsRef<str>,
        ValueT: AsRef<str>,
    {
        let mut headers_ = self.headers.clone();
        for (name, value) in headers {
            add_header(&mut headers_, name, value)?;
        }

        let mut url = url.to_string();

        let mut locations = HashSet::new();
        let mut redirections = 0;

        loop {
            let request = self.client.request(method.clone(), &url).headers(headers_.clone());
            let response = request.send().map_err(|error| error.to_string())?;

            if self.max_redirections == 0 {
                return Ok(response);
            }

            match is_redirect(&response)? {
                Some(location) => {
                    redirections += 1;
                    if redirections > self.max_redirections {
                        return Err(format!("too many redirections: {}", redirections));
                    }

                    if !locations.insert(location.clone()) {
                        return Err("redirection loop".into());
                    }

                    url = location;
                }

                None => {
                    return Ok(response);
                }
            }
        }
    }
}

// Utils

fn add_header<NameT, ValueT>(
    headers: &mut Vec<(HeaderName, HeaderValue)>,
    name: NameT,
    value: ValueT,
) -> Result<(), DispatchError>
where
    NameT: AsRef<str>,
    ValueT: AsRef<str>,
{
    let name = HeaderName::from_bytes(name.as_ref().as_bytes()).map_err(|error| error.to_string())?;
    let value = HeaderValue::from_bytes(value.as_ref().as_bytes()).map_err(|error| error.to_string())?;
    headers.push((name, value));
    Ok(())
}

fn is_redirect(response: &Response) -> Result<Option<String>, DispatchError> {
    let status = response.status_code();
    Ok(
        if (status >= 300)
            && (status < 400)
            && let Some(location) = response.header("location")
        {
            let location = location.to_str().map_err(|error| error.to_string())?;
            Some(location.into())
        } else {
            None
        },
    )
}
