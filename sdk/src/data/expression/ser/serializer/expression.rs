use super::{
    super::super::{super::super::dispatch_bindings::*, custom::*},
    error::*,
    list::*,
    map::*,
};

use serde::ser::*;

//
// ExpressionSerializer
//

/// Expression serializer.
#[derive(Debug, Default)]
pub struct ExpressionSerializer {
    /// Prefix for custom.
    pub custom_prefix: Option<String>,
}

impl ExpressionSerializer {
    /// Constructor.
    pub fn new(custom_prefix: Option<String>) -> Self {
        Self { custom_prefix }
    }

    /// With custom prefix.
    pub fn with_custom_prefix(&self, custom_kind: &'static str) -> String {
        match &self.custom_prefix {
            Some(custom_prefix) => custom_prefix.clone() + custom_kind,
            None => custom_kind.into(),
        }
    }
}

impl<'ser> Serializer for &'ser ExpressionSerializer {
    type Ok = Expression;
    type Error = SerializeError;
    type SerializeSeq = ListSerializer<'ser>;
    type SerializeTuple = ListSerializer<'ser>;
    type SerializeTupleStruct = ListSerializer<'ser>;
    type SerializeTupleVariant = ListSerializer<'ser>;
    type SerializeMap = MapSerializer<'ser>;
    type SerializeStruct = MapSerializer<'ser>;
    type SerializeStructVariant = MapSerializer<'ser>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Expression::Boolean(value))
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Expression::Integer(value as i64))
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Expression::Integer(value as i64))
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Expression::Integer(value as i64))
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Expression::Integer(value))
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Expression::UnsignedInteger(value as u64))
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Expression::UnsignedInteger(value as u64))
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Expression::UnsignedInteger(value as u64))
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Expression::UnsignedInteger(value))
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Expression::Float(value as f64))
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Expression::Float(value))
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        Ok(Expression::Text(value.into()))
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Expression::Text(value.into()))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Expression::Blob(value.into()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        // Note: we don't distinguish between None and ()
        self.serialize_unit()
    }

    fn serialize_some<SerializeT>(self, value: &SerializeT) -> Result<Self::Ok, Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        // Note: we lose the fact that this was a Some
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        // Note: we don't distinguish between None and ()
        Ok(().into())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Custom::new(name.into(), Expression::Null).into())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit_struct(name)
    }

    fn serialize_newtype_struct<SerializeT>(
        self,
        name: &'static str,
        value: &SerializeT,
    ) -> Result<Self::Ok, Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        Ok(Custom::new(name.into(), value.serialize(self)?).into())
    }

    fn serialize_newtype_variant<SerializeT>(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &SerializeT,
    ) -> Result<Self::Ok, Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        self.serialize_newtype_struct(name, value)
    }

    fn serialize_seq(self, length: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(ListSerializer::new(self, None, length))
    }

    fn serialize_tuple(self, length: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(length))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        length: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(ListSerializer::new(self, Some(name.into()), Some(length)))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        length: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_tuple_struct(name, length)
    }

    fn serialize_map(self, _length: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer::new(self, None))
    }

    fn serialize_struct(self, name: &'static str, _length: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(MapSerializer::new(self, Some(name.into())))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        length: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_struct(name, length)
    }
}
