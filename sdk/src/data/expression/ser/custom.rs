use super::super::super::super::dispatch_bindings::*;

use serde::ser::*;

impl Serialize for CustomResource {
    fn serialize<SerializerT>(&self, _serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        todo!();
    }
}
