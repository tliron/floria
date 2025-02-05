use super::super::{
    super::{errors::*, plugins::*, store::*},
    edge::*,
    propagation::*,
};

use kutil::std::error::*;

impl Edge {
    /// Update.
    pub fn update<StoreT, ErrorRecipientT>(
        &mut self,
        propagation: &mut Propagation,
        library: &mut Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorRecipientT: ErrorRecipient<FloriaError>,
    {
        if self.instance.update(library, errors)? {
            library.store.add_edge(self.clone())?;
        }

        if self.instance.prepare(library, errors)? {
            library.store.add_edge(self.clone())?;
        }

        if propagation.source_vertex {
            if propagation.should(&self.source_vertex_id) {
                if let Some(mut source_vertex) = library.store.get_vertex(&self.source_vertex_id)? {
                    source_vertex.update(propagation, library, errors)?;
                }
            }
        }

        if propagation.target_vertex {
            if propagation.should(&self.target_vertex_id) {
                if let Some(mut target_vertex) = library.store.get_vertex(&self.target_vertex_id)? {
                    target_vertex.update(propagation, library, errors)?;
                }
            }
        }

        Ok(())
    }
}
