use super::{call::*, expression::*};

use {
    duplicate::*,
    kutil::std::immutable::*,
    ordered_float::*,
    std::{borrow::*, collections::*},
};

#[duplicate_item(
  FromT                              Variant;
  [i64]                              [Integer];
  [u64]                              [UnsignedInteger];
  [OrderedFloat<f64>]                [Float];
  [f64]                              [Float];
  [bool]                             [Boolean];
  [ByteString]                       [Text];
  [String]                           [Text];
  [Bytes]                            [Blob];
  [Vec<Expression>]                  [List];
  [BTreeMap<Expression, Expression>] [Map];
  [Call]                             [Call];
)]
impl From<FromT> for Expression {
    fn from(value: FromT) -> Self {
        Self::Variant(value.into())
    }
}

impl From<&'static str> for Expression {
    fn from(value: &'static str) -> Self {
        Self::Text(ByteString::from_static(value))
    }
}

impl From<Cow<'_, str>> for Expression {
    fn from(value: Cow<'_, str>) -> Self {
        match value {
            Cow::Borrowed(string) => ByteString::from(string).into(),
            Cow::Owned(string) => string.into(),
        }
    }
}

impl From<&'static [u8]> for Expression {
    fn from(value: &'static [u8]) -> Self {
        Self::Blob(Bytes::from_static(value))
    }
}

impl From<Vec<u8>> for Expression {
    fn from(value: Vec<u8>) -> Self {
        Self::Blob(value.into())
    }
}
