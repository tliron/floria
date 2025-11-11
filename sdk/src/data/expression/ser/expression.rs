use super::super::super::super::dispatch_bindings::*;

use serde::ser::*;

impl Serialize for Expression {
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        match self {
            Self::Null => serializer.serialize_unit(),
            Self::Integer(integer) => integer.serialize(serializer),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.serialize(serializer),
            Self::Float(float) => float.serialize(serializer),
            Self::Boolean(boolean) => boolean.serialize(serializer),
            Self::Text(text) => text.serialize(serializer),
            Self::Blob(blob) => blob.serialize(serializer),
            Self::List(list) => list.serialize(serializer),
            Self::Map(map) => map.serialize(serializer),
            Self::Custom(custom) => custom.serialize(serializer),
            Self::Call(call) => call.serialize(serializer),
        }
    }
}
