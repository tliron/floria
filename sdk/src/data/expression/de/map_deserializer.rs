use super::{super::super::super::dispatch_bindings::*, deserializer::*, error::*};

use {serde::de, std::collections::*};

//
// MapDeserializer
//

pub(crate) struct MapDeserializer<'de> {
    iterator: btree_map::Iter<'de, Expression, Expression>,
    current_entry: Option<(&'de Expression, &'de Expression)>,
}

impl<'de> MapDeserializer<'de> {
    pub(crate) fn new(map: &'de MapResource) -> Self {
        Self { iterator: map.map().inner.iter(), current_entry: None }
    }

    fn next(&mut self) {
        self.current_entry = self.iterator.next();
    }
}

impl<'de> de::MapAccess<'de> for MapDeserializer<'de> {
    type Error = DeserializeError;

    fn next_key_seed<SeedT>(&mut self, seed: SeedT) -> Result<Option<SeedT::Value>, Self::Error>
    where
        SeedT: de::DeserializeSeed<'de>,
    {
        self.next();
        match self.current_entry {
            Some((key, _)) => Ok(Some(seed.deserialize(&mut ExpressionDeserializer::new(key))?)),
            None => Ok(None),
        }
    }

    fn next_value_seed<SeedT>(&mut self, seed: SeedT) -> Result<SeedT::Value, Self::Error>
    where
        SeedT: de::DeserializeSeed<'de>,
    {
        match self.current_entry {
            Some((_, value)) => Ok(seed.deserialize(&mut ExpressionDeserializer::new(value))?),
            None => Err(DeserializeError("no more items".into())), // this shouldn't happen, but still
        }
    }
}
