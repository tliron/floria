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
    fn metadata(&self) -> Vec<(floria_bindings::Expression, floria_bindings::Expression)> {
        self.metadata.get()
    }

    fn class_ids(&self) -> &Vec<floria_bindings::Id> {
        &self.class_ids
    }
}
