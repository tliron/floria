use super::{
    super::{data::*, floria_bindings},
    instance::*,
    metadata::*,
};

impl Metadata for floria_bindings::Vertex {
    fn metadata(&self) -> &floria_bindings::MapResource {
        &self.metadata
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

    fn properties_mut(&mut self) -> &mut Vec<(String, floria_bindings::Property)> {
        &mut self.properties
    }
}
