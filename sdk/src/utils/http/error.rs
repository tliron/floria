use crate::utils::escape_depiction_markup;

use super::super::super::plugin::*;

use {std::fmt, waki::*};

//
// HttpError
//

/// HTTP error
pub struct HttpError {
    /// Message.
    pub message: String,

    /// Status code.
    pub status_code: u16,

    /// Body.
    pub body: Option<Vec<u8>>,
}

impl fmt::Display for HttpError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.message, formatter)
    }
}

impl From<HttpError> for DispatchError {
    fn from(error: HttpError) -> Self {
        escape_depiction_markup(error.message)
    }
}

impl From<DispatchError> for HttpError {
    fn from(message: DispatchError) -> Self {
        Self { message, status_code: 0, body: None }
    }
}

impl From<Response> for HttpError {
    fn from(response: Response) -> Self {
        let status_code = response.status_code();
        let message = format!("HTTP status {}", status_code);
        let body = response.body().ok();
        Self { message, status_code, body }
    }
}
