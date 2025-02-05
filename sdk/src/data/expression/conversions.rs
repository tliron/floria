use super::{super::super::dispatch_bindings::*, call::*, custom::*, list::*, map::*};

use {duplicate::*, std::collections::*};

impl From<List> for Expression {
    fn from(list: List) -> Self {
        Self::List(ListResource::new(list))
    }
}

impl From<Map> for Expression {
    fn from(map: Map) -> Self {
        Self::Map(MapResource::new(map))
    }
}

impl From<Custom> for Expression {
    fn from(custom: Custom) -> Self {
        Self::Custom(CustomResource::new(custom))
    }
}

impl From<Call> for Expression {
    fn from(call: Call) -> Self {
        Self::Call(CallResource::new(call))
    }
}

// Conversion from primitives

impl From<()> for Expression {
    fn from(_null: ()) -> Self {
        Self::Null
    }
}

#[duplicate_item(
  FromT;
  [i64];
  [i32];
  [i16];
  [i8];
  [isize];
)]
impl From<FromT> for Expression {
    fn from(integer: FromT) -> Self {
        Self::Integer(integer as i64)
    }
}

#[duplicate_item(
  FromT;
  [u64];
  [u32];
  [u16];
  [u8];
  [usize];
)]
impl From<FromT> for Expression {
    fn from(unsigned_integer: FromT) -> Self {
        Self::UnsignedInteger(unsigned_integer as u64)
    }
}

#[duplicate_item(
  FromT;
  [f64];
  [f32];
)]
impl From<FromT> for Expression {
    fn from(float: FromT) -> Self {
        Self::Float(float as f64)
    }
}

impl From<bool> for Expression {
    fn from(boolean: bool) -> Self {
        Self::Boolean(boolean)
    }
}

impl From<String> for Expression {
    fn from(string: String) -> Self {
        Self::Text(string)
    }
}

impl From<&str> for Expression {
    fn from(string: &str) -> Self {
        Self::Text(string.into())
    }
}

impl From<Vec<u8>> for Expression {
    fn from(bytes: Vec<u8>) -> Self {
        Self::Blob(bytes)
    }
}

impl From<&[u8]> for Expression {
    fn from(bytes: &[u8]) -> Self {
        Self::Blob(bytes.into())
    }
}

impl From<Vec<Expression>> for Expression {
    fn from(vector: Vec<Expression>) -> Self {
        List::from(vector).into()
    }
}

impl From<BTreeMap<Expression, Expression>> for Expression {
    fn from(map: BTreeMap<Expression, Expression>) -> Self {
        Map::from(map).into()
    }
}

impl FromIterator<Expression> for Expression {
    fn from_iter<IntoIteratorT>(iter: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = Expression>,
    {
        List::from_iter(iter).into()
    }
}

impl FromIterator<(Expression, Expression)> for Expression {
    fn from_iter<IntoIteratorT>(iter: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = (Expression, Expression)>,
    {
        Map::from_iter(iter).into()
    }
}
