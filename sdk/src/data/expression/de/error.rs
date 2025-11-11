use {
    serde::*,
    std::{error::*, fmt},
};

//
// DeserializeError
//

/// Deserialize error.
#[derive(Debug)]
pub struct DeserializeError(pub String);

impl fmt::Display for DeserializeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

impl Error for DeserializeError {}

impl de::Error for DeserializeError {
    fn custom<DisplayT>(message: DisplayT) -> Self
    where
        DisplayT: fmt::Display,
    {
        Self(message.to_string())
    }
}
