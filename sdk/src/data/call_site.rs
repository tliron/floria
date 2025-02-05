use super::{
    super::{dispatch_bindings::*, host},
    entity::*,
    expression::*,
};

use std::fmt;

impl CallSite {
    /// Vertex or edge.
    pub fn entity(&self) -> Result<Entity, String> {
        Ok(Entity::new(self.id.clone(), host::get_entity(&self.id)?))
    }

    /// Site value.
    pub fn value(&self) -> Result<Option<Expression>, String> {
        if let Some(property_name) = self.path.first() {
            let entity = self.entity()?;
            if let Some(property) = entity.get_property(property_name)?
                && let Some(value) = property.value()
            {
                // TODO: dive into property segments? not here?
                return Ok(Some(value.clone()));
            }
        }

        Ok(None)
    }

    /// Get property metadata string.
    pub fn get_property_metadata_string(&self, name: &str) -> Result<Option<String>, String> {
        if let Some(property_name) = self.path.first() {
            let entity = self.entity()?;
            if let Some(property) = entity.get_property(property_name)? {
                return Ok(property.get_metadata_string(name)?.map(|value| value.into()));
            }
        }

        Ok(None)
    }

    /// Get property metadata map.
    pub fn get_property_metadata_map(&self, name: &str) -> Result<Option<Map>, String> {
        if let Some(property_name) = self.path.first() {
            let entity = self.entity()?;
            if let Some(property) = entity.get_property(property_name)? {
                return Ok(property.get_metadata_map(name)?.map(|value| value.clone()));
            }
        }

        Ok(None)
    }
}

impl fmt::Display for CallSite {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.id, formatter)?;
        for segment in &self.path {
            write!(formatter, ".{}", segment)?;
        }
        Ok(())
    }
}
