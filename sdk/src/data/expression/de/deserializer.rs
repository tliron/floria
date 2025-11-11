use super::{
    super::super::super::dispatch_bindings::*, enum_deserializer::*, error::*, map_as_list_deserializer::*,
    map_deserializer::*, seq_deserializer::*,
};

use {num_traits::*, serde::de};

//
// ExpressionDeserializer
//

/// Serde deserializer for expressions.
///
/// Will convert number types only if information is not lost. Otherwise, will return an error.
///
/// See [NumCast::from](cast::NumCast::from).
pub struct ExpressionDeserializer<'inner> {
    inner: &'inner Expression,
}

impl<'inner> ExpressionDeserializer<'inner> {
    /// Constructor
    pub fn new(inner: &'inner Expression) -> Self {
        Self { inner }
    }

    fn incompatible_type_error(&self) -> DeserializeError {
        DeserializeError(format!("incompatible type: |error|{}|", self.inner.type_name()))
    }

    fn incompatible_value_error(&self) -> DeserializeError {
        DeserializeError(format!("incompatible value: |error|{}|", self.inner.type_name()))
    }
}

// See: https://serde.rs/impl-deserializer.html

impl<'de, 'this> de::Deserializer<'de> for &'this mut ExpressionDeserializer<'de> {
    type Error = DeserializeError;

    fn deserialize_any<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::Null => self.deserialize_unit(visitor),
            Expression::Integer(_) => self.deserialize_i64(visitor),
            Expression::UnsignedInteger(_) => self.deserialize_u64(visitor),
            Expression::Float(_) => self.deserialize_f64(visitor),
            Expression::Boolean(_) => self.deserialize_bool(visitor),
            Expression::Text(_) => self.deserialize_str(visitor),
            Expression::Blob(_) => self.deserialize_bytes(visitor),
            Expression::List(_) => self.deserialize_seq(visitor),
            Expression::Map(_) => self.deserialize_map(visitor),
            Expression::Custom(_) => self.deserialize_map(visitor),
            Expression::Call(_) => self.deserialize_map(visitor),
        }
    }

    fn deserialize_bool<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::Boolean(boolean) => visitor.visit_bool(*boolean),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_i8<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::Integer(integer) => match cast(*integer) {
                Some(integer) => visitor.visit_i8(integer),
                None => Err(self.incompatible_value_error()),
            },

            Expression::UnsignedInteger(unsigned_integer) => match cast(*unsigned_integer) {
                Some(integer) => visitor.visit_i8(integer),
                None => Err(self.incompatible_value_error()),
            },

            Expression::Float(float) => {
                if float.fract() == 0. {
                    match cast(*float) {
                        Some(integer) => visitor.visit_i8(integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_i16<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::Integer(integer) => match cast(*integer) {
                Some(integer) => visitor.visit_i16(integer),
                None => Err(self.incompatible_value_error()),
            },

            Expression::UnsignedInteger(unsigned_integer) => match cast(*unsigned_integer) {
                Some(integer) => visitor.visit_i16(integer),
                None => Err(self.incompatible_value_error()),
            },

            Expression::Float(float) => {
                if float.fract() == 0. {
                    match cast(*float) {
                        Some(integer) => visitor.visit_i16(integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_i32<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::Integer(integer) => match cast(*integer) {
                Some(integer) => visitor.visit_i32(integer),
                None => Err(self.incompatible_value_error()),
            },

            Expression::UnsignedInteger(unsigned_integer) => match cast(*unsigned_integer) {
                Some(integer) => visitor.visit_i32(integer),
                None => Err(self.incompatible_value_error()),
            },

            Expression::Float(float) => {
                if float.fract() == 0. {
                    match cast(*float) {
                        Some(integer) => visitor.visit_i32(integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_i64<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::Integer(integer) => visitor.visit_i64(*integer),

            Expression::UnsignedInteger(unsigned_integer) => match cast(*unsigned_integer) {
                Some(integer) => visitor.visit_i64(integer),
                None => Err(self.incompatible_value_error()),
            },

            Expression::Float(float) => {
                if float.fract() == 0. {
                    match cast(*float) {
                        Some(integer) => visitor.visit_i64(integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_u8<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::UnsignedInteger(unsigned_integer) => match cast(*unsigned_integer) {
                Some(unsigned_integer) => visitor.visit_u8(unsigned_integer),
                None => Err(self.incompatible_value_error()),
            },

            Expression::Integer(integer) => {
                if *integer >= 0 {
                    match cast(*integer) {
                        Some(insigned_integer) => visitor.visit_u8(insigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            Expression::Float(float) => {
                if (*float >= 0.) && (float.fract() == 0.) {
                    match cast(*float) {
                        Some(unsigned_integer) => visitor.visit_u8(unsigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_u16<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::UnsignedInteger(unsigned_integer) => match cast(*unsigned_integer) {
                Some(unsigned_integer) => visitor.visit_u16(unsigned_integer),
                None => Err(self.incompatible_value_error()),
            },

            Expression::Integer(integer) => {
                if *integer >= 0 {
                    match cast(*integer) {
                        Some(insigned_integer) => visitor.visit_u16(insigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            Expression::Float(float) => {
                if (*float >= 0.) && (float.fract() == 0.) {
                    match cast(*float) {
                        Some(unsigned_integer) => visitor.visit_u16(unsigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_u32<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::UnsignedInteger(unsigned_integer) => match cast(*unsigned_integer) {
                Some(unsigned_integer) => visitor.visit_u32(unsigned_integer),
                None => Err(self.incompatible_value_error()),
            },

            Expression::Integer(integer) => {
                if *integer >= 0 {
                    match cast(*integer) {
                        Some(insigned_integer) => visitor.visit_u32(insigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            Expression::Float(float) => {
                if (*float >= 0.) && (float.fract() == 0.) {
                    match cast(*float) {
                        Some(unsigned_integer) => visitor.visit_u32(unsigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_u64<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::UnsignedInteger(unsigned_integer) => visitor.visit_u64(*unsigned_integer),

            Expression::Integer(integer) => {
                if *integer >= 0 {
                    match cast(*integer) {
                        Some(insigned_integer) => visitor.visit_u64(insigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            Expression::Float(float) => {
                if (*float >= 0.) && (float.fract() == 0.) {
                    match cast(*float) {
                        Some(unsigned_integer) => visitor.visit_u64(unsigned_integer),
                        None => Err(self.incompatible_value_error()),
                    }
                } else {
                    Err(self.incompatible_value_error())
                }
            }

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_f32<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::Float(float) => match cast(*float) {
                Some(float) => visitor.visit_f32(float),
                None => Err(self.incompatible_value_error()),
            },

            Expression::Integer(integer) => match cast(*integer) {
                Some(float) => visitor.visit_f32(float),
                None => Err(self.incompatible_value_error()),
            },

            Expression::UnsignedInteger(unsigned_integer) => match cast(*unsigned_integer) {
                Some(float) => visitor.visit_f32(float),
                None => Err(self.incompatible_value_error()),
            },

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_f64<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::Float(float) => visitor.visit_f64(*float),

            Expression::Integer(integer) => match cast(*integer) {
                Some(float) => visitor.visit_f64(float),
                None => Err(self.incompatible_value_error()),
            },

            Expression::UnsignedInteger(unsigned_integer) => match cast::<_, f64>(*unsigned_integer) {
                Some(float) => visitor.visit_f64(float),
                None => Err(self.incompatible_value_error()),
            },

            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_char<VisitorT>(self, _visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        Err(DeserializeError("not supported: |error|deserialize_char|".into()))
    }

    fn deserialize_str<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::Text(text) => visitor.visit_str(text),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_string<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::Text(text) => visitor.visit_str(text),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_bytes<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::Blob(blob) => visitor.visit_bytes(blob),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_byte_buf<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::Blob(blob) => visitor.visit_bytes(blob),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_option<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::Null => visitor.visit_unit(),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_unit_struct<VisitorT>(
        self,
        _name: &'static str,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<VisitorT>(
        self,
        _name: &'static str,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::List(list) => Ok(visitor.visit_seq(SeqDeserializer::new(list))?),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_tuple<VisitorT>(self, _len: usize, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<VisitorT>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::Map(map) => Ok(visitor.visit_map(MapDeserializer::new(map))?),
            Expression::List(list) => Ok(visitor.visit_map(MapAsListDeserializer::new(list))?),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_struct<VisitorT>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<VisitorT>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        match self.inner {
            Expression::Map(map) => Ok(visitor.visit_enum(EnumDeserializer::new(map)?)?),
            _ => Err(self.incompatible_type_error()),
        }
    }

    fn deserialize_identifier<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_ignored_any<VisitorT>(self, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}
