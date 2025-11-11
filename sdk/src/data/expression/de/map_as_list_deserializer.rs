use super::{super::super::super::dispatch_bindings::*, deserializer::*, error::*};

use {serde::de, std::slice::*};

//
// MapAsListDeserializer
//

pub(crate) struct MapAsListDeserializer<'de> {
    iterator: Iter<'de, Expression>,
    current_entry: Option<(&'de Expression, &'de Expression)>,
}

impl<'de> MapAsListDeserializer<'de> {
    pub(crate) fn new(list: &'de ListResource) -> Self {
        Self { iterator: list.list().inner.iter(), current_entry: None }
    }

    fn next(&mut self) -> Result<(), DeserializeError> {
        let current_entry = self.iterator.next();

        match current_entry {
            Some(current_entry) => {
                self.current_entry = match current_entry {
                    Expression::List(list) => {
                        let list = &list.list().inner;
                        let length = list.len();
                        if length == 2 {
                            let mut iter = list.into_iter();
                            Some((iter.next().expect("first"), iter.next().expect("second")))
                        } else {
                            return Err(DeserializeError(format!("length is not 2: |error|{}|", length)));
                        }
                    }
                    _ => None,
                };

                if self.current_entry.is_none() {
                    return Err(DeserializeError(format!(
                        "incompatible variant: |error|{}|",
                        current_entry.type_name()
                    )));
                }
            }

            None => {
                self.current_entry = None;
            }
        }

        Ok(())
    }
}

impl<'de> de::MapAccess<'de> for MapAsListDeserializer<'de> {
    type Error = DeserializeError;

    fn next_key_seed<SeedT>(&mut self, seed: SeedT) -> Result<Option<SeedT::Value>, Self::Error>
    where
        SeedT: de::DeserializeSeed<'de>,
    {
        self.next()?;
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
