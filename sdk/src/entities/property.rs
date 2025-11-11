use super::{
    super::{data::*, floria_bindings},
    metadata::*,
};

impl floria_bindings::Property {
    /// Value.
    pub fn value(&self) -> Option<Expression> {
        self.value.as_ref().map(|value| value.into())
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
