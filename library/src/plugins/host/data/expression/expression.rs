use super::super::super::super::bindings::floria::plugins::floria as bindings;

use {duplicate::*, kutil::std::immutable::*, ordered_float::*, wasmtime::component::*};

impl Default for bindings::Expression {
    fn default() -> Self {
        Self::Null
    }
}

#[duplicate_item(
  FromT                                Variant;
  [i64]                                [Integer];
  [u64]                                [UnsignedInteger];
  [f64]                                [Float];
  [bool]                               [Boolean];
  [String]                             [Text];
  [Vec<u8>]                            [Blob];
  [Resource<bindings::ListResource>]   [List];
  [Resource<bindings::MapResource>]    [Map];
  [Resource<bindings::CustomResource>] [Custom];
  [Resource<bindings::CallResource>]   [Call];
)]
impl From<FromT> for bindings::Expression {
    fn from(value: FromT) -> Self {
        Self::Variant(value)
    }
}

impl From<ByteString> for bindings::Expression {
    fn from(value: ByteString) -> Self {
        Self::Text(value.into())
    }
}

impl From<Bytes> for bindings::Expression {
    fn from(value: Bytes) -> Self {
        Self::Blob(value.into())
    }
}

impl From<OrderedFloat<f64>> for bindings::Expression {
    fn from(value: OrderedFloat<f64>) -> Self {
        Self::Float(value.into())
    }
}
