use super::super::super::plugin::*;

use waki::{header::*, *};

pub fn add_header<NameT, ValueT>(
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

pub fn is_redirect(response: &Response) -> Result<Option<String>, DispatchError> {
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

/// Log.
#[macro_export]
macro_rules! assert_success {
    ( $response:expr $(,)? ) => {
        let status = $response.status_code();
        if (status < 200) || (status >= 300) {
            return Err($response.into());
        }
    };
}

pub use assert_success;
