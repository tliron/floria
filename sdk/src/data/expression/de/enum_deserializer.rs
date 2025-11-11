use super::{super::super::super::dispatch_bindings::*, deserializer::*, error::*, variant_deserializer::*};

use serde::de;

//
// EnumDeserializer
//

pub(crate) struct EnumDeserializer<'de> {
    key: &'de Expression,
    value: &'de Expression,
}

impl<'de> EnumDeserializer<'de> {
    pub(crate) fn new(map: &'de MapResource) -> Result<Self, DeserializeError> {
        let map = &map.map().inner;
        let length = map.len();
        if length == 1 {
            let (key, value) = map.iter().next().expect("non-empty");
            Ok(Self { key, value })
        } else {
            Err(DeserializeError(format!("map length is not 1: {}", length)))
        }
    }
}

impl<'de> de::EnumAccess<'de> for EnumDeserializer<'de> {
    type Error = DeserializeError;
    type Variant = VariantDeserializer<'de>;

    fn variant_seed<SeedT>(self, seed: SeedT) -> Result<(SeedT::Value, Self::Variant), Self::Error>
    where
        SeedT: de::DeserializeSeed<'de>,
    {
        let key = seed.deserialize(&mut ExpressionDeserializer::new(self.key))?;
        Ok((key, VariantDeserializer::new(self.value)))
    }
}
