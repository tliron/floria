use std::fmt;

//
// Number
//

/// Number.
#[derive(Clone, Copy, Debug)]
pub enum Number {
    /// Integer.
    Integer(i64),

    /// Unsigned integer.
    UnsignedInteger(u64),

    /// Float.
    Float(f64),
}

impl Number {
    /// Type name.
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Integer(_) => "integer",
            Self::UnsignedInteger(_) => "unsigned integer",
            Self::Float(_) => "float",
        }
    }
}

impl Default for Number {
    fn default() -> Self {
        Self::Integer(0)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(integer) => fmt::Display::fmt(integer, formatter),
            Self::UnsignedInteger(unsigned_integer) => fmt::Display::fmt(unsigned_integer, formatter),
            Self::Float(float) => fmt::Display::fmt(float, formatter),
        }
    }
}
