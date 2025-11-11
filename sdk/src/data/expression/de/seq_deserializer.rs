use super::{super::super::super::dispatch_bindings::*, deserializer::*, error::*};

use {serde::de, std::slice::*};

//
// SeqDeserializer
//

pub(crate) struct SeqDeserializer<'de> {
    iterator: Iter<'de, Expression>,
    current_item: Option<&'de Expression>,
}

impl<'de> SeqDeserializer<'de> {
    pub(crate) fn new(list: &'de ListResource) -> Self {
        Self { iterator: list.list().inner.iter(), current_item: None }
    }

    fn next(&mut self) {
        self.current_item = self.iterator.next();
    }
}

impl<'de> de::SeqAccess<'de> for SeqDeserializer<'de> {
    type Error = DeserializeError;

    fn next_element_seed<SeedT>(&mut self, seed: SeedT) -> Result<Option<SeedT::Value>, Self::Error>
    where
        SeedT: de::DeserializeSeed<'de>,
    {
        self.next();
        match self.current_item {
            Some(item) => Ok(Some(seed.deserialize(&mut ExpressionDeserializer::new(item))?)),
            None => Ok(None),
        }
    }
}
