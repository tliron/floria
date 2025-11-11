use super::{super::super::plugin::*, error::*, utils::*};

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
    pub fn add_header<HeaderNameT, HeaderValueT>(
        &mut self,
        name: HeaderNameT,
        value: HeaderValueT,
    ) -> Result<(), DispatchError>
    where
        HeaderNameT: AsRef<str>,
        HeaderValueT: AsRef<str>,
    {
        add_header(&mut self.headers, name, value)
    }

    /// Get string.
    pub fn get_string<HeadersT, HeaderNameT, HeaderValueT>(
        &self,
        url: &str,
        headers: HeadersT,
    ) -> Result<String, HttpError>
    where
        HeadersT: IntoIterator<Item = (HeaderNameT, HeaderValueT)>,
        HeaderNameT: AsRef<str>,
        HeaderValueT: AsRef<str>,
    {
        let body = self.get_bytes(url, headers)?;
        let body = String::from_utf8(body).map_err(|error| error.to_string())?;
        Ok(body)
    }

    /// Get bytes.
    pub fn get_bytes<HeadersT, HeaderNameT, HeaderValueT>(
        &self,
        url: &str,
        headers: HeadersT,
    ) -> Result<Vec<u8>, HttpError>
    where
        HeadersT: IntoIterator<Item = (HeaderNameT, HeaderValueT)>,
        HeaderNameT: AsRef<str>,
        HeaderValueT: AsRef<str>,
    {
        let response = self.request(Method::Get, url, headers, None)?;
        assert_success!(response);
        let body = response.body().map_err(|error| error.to_string())?;
        Ok(body)
    }

    /// Put bytes.
    pub fn put_bytes<HeadersT, HeaderNameT, HeaderValueT>(
        &self,
        url: &str,
        headers: HeadersT,
        body: &[u8],
    ) -> Result<Vec<u8>, HttpError>
    where
        HeadersT: IntoIterator<Item = (HeaderNameT, HeaderValueT)>,
        HeaderNameT: AsRef<str>,
        HeaderValueT: AsRef<str>,
    {
        let response = self.request(Method::Put, url, headers, Some(body))?;
        assert_success!(response);
        let body = response.body().map_err(|error| error.to_string())?;
        Ok(body)
    }

    /// Post bytes.
    pub fn post_bytes<HeadersT, HeaderNameT, HeaderValueT>(
        &self,
        url: &str,
        headers: HeadersT,
        body: &[u8],
    ) -> Result<Vec<u8>, HttpError>
    where
        HeadersT: IntoIterator<Item = (HeaderNameT, HeaderValueT)>,
        HeaderNameT: AsRef<str>,
        HeaderValueT: AsRef<str>,
    {
        let response = self.request(Method::Post, url, headers, Some(body))?;
        assert_success!(response);
        let body = response.body().map_err(|error| error.to_string())?;
        Ok(body)
    }

    /// Request.
    pub fn request<HeadersT, HeaderNameT, HeaderValueT>(
        &self,
        method: Method,
        url: &str,
        headers: HeadersT,
        body: Option<&[u8]>,
    ) -> Result<Response, DispatchError>
    where
        HeadersT: IntoIterator<Item = (HeaderNameT, HeaderValueT)>,
        HeaderNameT: AsRef<str>,
        HeaderValueT: AsRef<str>,
    {
        let mut headers_ = self.headers.clone();
        for (name, value) in headers {
            add_header(&mut headers_, name, value)?;
        }

        let mut url = url.to_string();

        let mut locations = HashSet::new();
        let mut redirections = 0;

        loop {
            let mut request = self.client.request(method.clone(), &url).headers(headers_.clone());
            if let Some(body) = body {
                request = request.body(body);
            }

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
