use super::{
    super::{data::*, floria_bindings},
    instance::*,
    metadata::*,
};

impl Metadata for floria_bindings::Vertex {
    fn metadata(&self) -> Vec<(floria_bindings::Expression, floria_bindings::Expression)> {
        self.metadata.get()
    }

    fn class_ids(&self) -> &Vec<floria_bindings::Id> {
        &self.class_ids
    }
}

impl Instance for floria_bindings::Vertex {
    fn id(&self) -> Id {
        self.id.clone().into()
    }

    fn properties(&self) -> &Vec<(String, floria_bindings::Property)> {
        &self.properties
    }
}
