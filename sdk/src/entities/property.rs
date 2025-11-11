use super::{
    super::{data::*, floria_bindings, utils::*, *},
    metadata::*,
};

impl floria_bindings::Property {
    /// Value.
    pub fn value(&self) -> Option<Expression> {
        self.value.as_ref().map(|value| value.into())
    }

    /// Value.
    pub fn must_value(&self, name: &str) -> Result<Expression, DispatchError> {
        self.value().ok_or_else(|| format!("missing |meta|property| value: |error|{}|", escape_depiction_markup(name)))
    }
}

impl Metadata for floria_bindings::Property {
    fn metadata(&self) -> &floria_bindings::MapResource {
        &self.metadata
    }

    fn class_ids(&self) -> &Vec<floria_bindings::Id> {
        &self.class_ids
    }
}
