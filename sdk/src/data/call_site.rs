use super::super::{dispatch_bindings::*, *};

use std::fmt;

impl CallSite {
    /// Constructor.
    pub fn new(id: Id, property: Option<String>) -> Self {
        Self { id, property }
    }

    /// Entity.
    pub fn entity(&self) -> Result<floria_bindings::Entity, DispatchError> {
        // TODO: cache it
        Ok(host::get_entity(&self.id)?)
    }

    /// Property value.
    pub fn property_value(&self) -> Result<Option<Expression>, DispatchError> {
        Ok(match &self.property {
            Some(property) => self.entity()?.property(property).and_then(|property| property.value()),
            None => None,
        })
    }

    // /// Property metadata string.
    // pub fn property_metadata_string(&self, key: &str) -> Result<Option<String>, DispatchError> {
    //     Ok(match &self.property {
    //         Some(property) => self.entity()?.property(property).and_then(|property| property.metadata_string(key)),
    //         None => None,
    //     })
    // }

    // /// Property metadata map.
    // pub fn property_metadata_map(&self, name: &str) -> Result<Option<Map>, String> {
    //     if let Some(property) = &self.property {
    //         let entity = self.entity()?;
    //         if let Some(property) = entity.property(property)? {
    //             return Ok(property.metadata_map(name)?.map(|value| value.clone()));
    //         }
    //     }

    //     Ok(None)
    // }
}

impl fmt::Display for CallSite {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.id, formatter)?;
        if let Some(property) = &self.property {
            write!(formatter, ".{}", property)?;
        }
        Ok(())
    }
}
