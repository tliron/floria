use super::{super::dispatch_bindings::*, expression::*};

use std::collections::*;

//
// Property
//

/// Property.
#[derive(Clone)]
pub struct Property<'own> {
    /// Name.
    pub name: String,

    /// Expression.
    pub expression: &'own Expression,
}

impl<'own> Property<'own> {
    /// Constructor.
    pub fn new(name: String, expression: &'own Expression) -> Self {
        Self { name, expression }
    }

    /// Value.
    pub fn value(&self) -> Option<&Expression> {
        self.expression.into_get("value")
    }

    /// Metadata.
    pub fn metadata(&self) -> Result<Option<&BTreeMap<Expression, Expression>>, String> {
        match self.expression.into_get("metadata") {
            Some(metadata) => match metadata {
                Expression::Map(metadata) => Ok(Some(&metadata.map().inner)),
                _ => Err(format!("property {}: malformed \"metadata\", not a map", self.name)),
            },

            _ => Ok(None),
        }
    }

    /// Get metadata string.
    pub fn get_metadata_string(&self, name: &str) -> Result<Option<&str>, String> {
        if let Some(metadata) = self.metadata()?
            && let Some(value) = metadata.get(&name.into())
            && let Expression::Text(value) = value
        {
            return Ok(Some(value));
        }

        Ok(None)
    }

    /// Get metadata map.
    pub fn get_metadata_map(&self, name: &str) -> Result<Option<&Map>, String> {
        if let Some(metadata) = self.metadata()?
            && let Some(value) = metadata.get(&name.into())
            && let Expression::Map(value) = value
        {
            return Ok(Some(value.map()));
        }

        Ok(None)
    }

    /// Read-only.
    pub fn is_read_only(&self) -> Result<bool, String> {
        let read_only = self
            .expression
            .into_get("read_only")
            .ok_or_else(|| format!("property {} is missing read_only", self.name))?;
        match read_only {
            Expression::Boolean(read_only) => Ok(*read_only),
            _ => Err(format!("property {}: malformed \"read_only\", not a boolean", self.name)),
        }
    }
}
