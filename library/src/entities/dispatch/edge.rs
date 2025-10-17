use super::super::{
    super::{errors::*, plugins::*, store::*},
    edge::*,
    propagation::*,
};

use kutil::std::error::*;

impl Edge {
    /// Update properties.
    pub fn update_properties<StoreT, ErrorReceiverT>(
        &mut self,
        propagation: &mut Propagation,
        library: &mut Library<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<(), FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        if self.instance.update_properties(library, errors)? {
            library.store.add_edge(self.clone())?;
        }

        if self.instance.prepare_properties(library, errors)? {
            library.store.add_edge(self.clone())?;
        }

        if propagation.source_vertex {
            if propagation.should(&self.source_vertex_id) {
                if let Some(mut source_vertex) = library.store.get_vertex(&self.source_vertex_id)? {
                    source_vertex.update_properties(propagation, library, errors)?;
                }
            }
        }

        if propagation.target_vertex {
            if propagation.should(&self.target_vertex_id) {
                if let Some(mut target_vertex) = library.store.get_vertex(&self.target_vertex_id)? {
                    target_vertex.update_properties(propagation, library, errors)?;
                }
            }
        }

        Ok(())
    }
}
