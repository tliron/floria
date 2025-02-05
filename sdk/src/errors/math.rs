use std::fmt;

/// Integer fit.
pub fn integer_fit<DisplayT>(number: DisplayT) -> String
where
    DisplayT: fmt::Display,
{
    format!("won't fit in integer: |error|{}|", number)
}

/// Unsigned integer fit.
pub fn unsigned_integer_fit<DisplayT>(number: DisplayT) -> String
where
    DisplayT: fmt::Display,
{
    format!("won't fit in unsigned integer: |error|{}|", number)
}

/// Float fit.
pub fn float_fit<DisplayT>(number: DisplayT) -> String
where
    DisplayT: fmt::Display,
{
    format!("won't fit in float: |error|{}|", number)
}

/// Integer overflow.
pub fn integer_overflow(left: i64, right: i64, delimiter: &str) -> String {
    format!("integer overflow: |error|{} {} {}|", left, delimiter, right)
}

/// Unsigned integer overflow.
pub fn unsigned_integer_overflow(left: u64, right: u64, delimiter: &str) -> String {
    format!("unsigned integer overflow: |error|{} {} {}|", left, delimiter, right)
}
