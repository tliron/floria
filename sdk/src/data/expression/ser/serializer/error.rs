use {
    serde::*,
    std::{error::*, fmt},
};

//
// SerializeError
//

/// Serialize error.
#[derive(Debug)]
pub struct SerializeError(pub String);

impl fmt::Display for SerializeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

impl Error for SerializeError {}

impl ser::Error for SerializeError {
    fn custom<DisplayT>(message: DisplayT) -> Self
    where
        DisplayT: fmt::Display,
    {
        Self(message.to_string())
    }
}
