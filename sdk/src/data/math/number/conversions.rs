use super::{
    super::super::super::{dispatch_bindings::*, utils::*},
    number::*,
};

use {num_traits::*, std::str::*};

impl From<i64> for Number {
    fn from(integer: i64) -> Self {
        Self::Integer(integer)
    }
}

impl From<u64> for Number {
    fn from(unsigned_integer: u64) -> Self {
        Self::UnsignedInteger(unsigned_integer)
    }
}

impl From<f64> for Number {
    fn from(float: f64) -> Self {
        Self::Float(float)
    }
}

impl FromStr for Number {
    type Err = String;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        if let Ok(unsigned_integer) = representation.parse() {
            return Ok(Self::UnsignedInteger(unsigned_integer));
        }

        if let Ok(integer) = representation.parse() {
            return Ok(Self::Integer(integer));
        }

        if let Ok(float) = representation.parse() {
            return Ok(Self::Float(float));
        }

        Err(format!("not a number: |error|{:?}|", escape_depiction_markup(representation)).into())
    }
}

impl TryFrom<&Expression> for Number {
    type Error = String;

    fn try_from(expression: &Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::Integer(integer) => Ok(Self::Integer(*integer)),
            Expression::UnsignedInteger(unsigned_integer) => Ok(Self::UnsignedInteger(*unsigned_integer)),
            Expression::Float(float) => Ok(Self::Float(*float)),

            _ => Err(format!(
                "not an |name|integer|, |name|unsigned integer|, or |name|float|: |error|{}|",
                expression.type_name()
            )),
        }
    }
}

impl TryFrom<Expression> for Number {
    type Error = String;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        (&expression).try_into()
    }
}

impl TryInto<i64> for Number {
    type Error = String;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Self::Integer(integer) => Ok(integer),

            Self::UnsignedInteger(unsigned_integer) => match cast(unsigned_integer) {
                Some(integer) => Ok(integer),
                None => Err(format!("won't fit in integer: |error|{}|", unsigned_integer)),
            },

            Self::Float(float) => match cast(float) {
                Some(integer) => Ok(integer),
                None => Err(format!("won't fit in integer: |error|{}|", float)),
            },
        }
    }
}

impl TryInto<u64> for Number {
    type Error = String;

    fn try_into(self) -> Result<u64, Self::Error> {
        match self {
            Self::Integer(integer) => match cast(integer) {
                Some(unsigned_integer) => Ok(unsigned_integer),
                None => Err(format!("won't fit in unsigned integer: |error|{}|", integer)),
            },

            Self::UnsignedInteger(unsigned_integer) => Ok(unsigned_integer),

            Self::Float(float) => match cast(float) {
                Some(unsigned_integer) => Ok(unsigned_integer),
                None => Err(format!("won't fit in unsigned integer: |error|{}|", float)),
            },
        }
    }
}

impl TryInto<f64> for Number {
    type Error = String;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Self::Integer(integer) => match cast(integer) {
                Some(float) => Ok(float),
                None => Err(format!("won't fit in float: |error|{}|", integer)),
            },

            Self::UnsignedInteger(unsigned_integer) => match cast(unsigned_integer) {
                Some(float) => Ok(float),
                None => Err(format!("won't fit in float: |error|{}|", unsigned_integer)),
            },

            Self::Float(float) => Ok(float),
        }
    }
}

impl Into<Expression> for Number {
    fn into(self) -> Expression {
        match self {
            Self::Integer(integer) => integer.into(),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.into(),
            Self::Float(float) => float.into(),
        }
    }
}
