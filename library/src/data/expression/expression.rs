use super::super::call::*;

use {kutil::std::immutable::*, ordered_float::*, std::collections::*};

//
// Expression
//

/// Expression.
#[derive(Clone, Debug, Default)]
pub enum Expression {
    /// Undefined.
    #[default]
    Undefined,

    /// Null.
    Null,

    /// Integer.
    Integer(i64),

    /// Unsigned integer.
    UnsignedInteger(u64),

    /// Float.
    Float(OrderedFloat<f64>),

    /// Boolean.
    Boolean(bool),

    /// Text.
    Text(ByteString),

    /// Blob.
    Blob(Bytes),

    /// List.
    List(Vec<Expression>),

    /// Map.
    Map(BTreeMap<Expression, Expression>),

    /// Custom.
    Custom(ByteString, Box<Expression>),

    /// Call.
    Call(Call),
}

impl Expression {
    /// True if has no calls.
    pub fn is_literal(&self) -> bool {
        match self {
            Expression::List(list) => {
                for item in list {
                    if !item.is_literal() {
                        return false;
                    }
                }
                true
            }

            Expression::Map(map) => {
                for (key, value) in map {
                    if !key.is_literal() {
                        return false;
                    }
                    if !value.is_literal() {
                        return false;
                    }
                }
                true
            }

            Expression::Custom(_kind, inner) => inner.is_literal(),

            Expression::Call(_) => false,

            _ => true,
        }
    }
}
