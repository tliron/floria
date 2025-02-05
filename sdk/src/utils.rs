use super::data::*;

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

/// Assert argument count.
pub fn assert_argument_count(arguments: &Vec<Expression>, count: usize) -> Result<(), String> {
    let length = arguments.len();
    if length != count {
        let s = if count == 1 { "" } else { "s" };
        return Err(format!("must have {} argument{}: |error|{}|", count_to_string(count), s, length));
    }
    Ok(())
}

/// Assert argument count.
pub fn assert_argument_count_min(arguments: &Vec<Expression>, min: usize) -> Result<(), String> {
    let length = arguments.len();
    if length < min {
        let s = if min == 1 { "" } else { "s" };
        return Err(format!("must have at least {} argument{}: |error|{}|", count_to_string(min), s, length));
    }
    Ok(())
}

/// Assert argument count.
pub fn assert_argument_count_range(arguments: &Vec<Expression>, min: usize, max: usize) -> Result<(), String> {
    let length = arguments.len();
    if (length < min) || (length > max) {
        let s = if max == 1 { "" } else { "s" };
        if min == 0 {
            return Err(format!("must have at most {} argument{}: |error|{}|", count_to_string(max), s, length));
        } else {
            return Err(format!(
                "must have between {} and {} argument{}: |error|{}|",
                count_to_string(min),
                count_to_string(max),
                s,
                length
            ));
        }
    }
    Ok(())
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

// Utils

fn count_to_string(count: usize) -> String {
    match count {
        0 => "no".to_string(),
        _ => count.to_string(),
    }
}
