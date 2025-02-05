use std::fmt;

/// Depiction markup delimiter.
pub const MARKUP_DELIMITER: char = '|';

/// Depiction markup escaped delimiter.
pub const MARKUP_ESCAPED_DELIMITER: &str = "\\|";

/// Escape depiction markup.
pub fn escape_depiction_markup<ThingT>(thing: ThingT) -> String
where
    ThingT: fmt::Display,
{
    thing.to_string().replace(MARKUP_DELIMITER, MARKUP_ESCAPED_DELIMITER)
}

//
// MapEscapeDepictionError
//

/// Map escape depiction error.
pub trait MapEscapeDepictionError<OkT> {
    /// Map escape depiction error.
    fn map_escape_depiction_error(self) -> Result<OkT, String>;
}

impl<OkT, FromErrorT> MapEscapeDepictionError<OkT> for Result<OkT, FromErrorT>
where
    FromErrorT: fmt::Display,
{
    fn map_escape_depiction_error(self) -> Result<OkT, String> {
        self.map_err(|error| escape_depiction_markup(error))
    }
}
