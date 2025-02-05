use super::super::{
    super::{
        super::{errors::*, plugins::*, store::*},
        directory::*,
    },
    vertex::*,
};

use kutil::std::error::*;

impl Vertex {
    /// Update.
    pub fn update<StoreT, ErrorRecipientT>(
        &mut self,
        library: &mut Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorRecipientT: ErrorRecipient<FloriaError>,
    {
        if self.instance.update(library, errors)? {
            library.store.add_vertex(self.clone())?;
            if self.instance.prepare(library, errors)? {
                library.store.add_vertex(self.clone())?;
            }
        }

        for vertex_id in &self.contained_vertex_ids {
            if let Some(mut vertex) = library.store.get_vertex(vertex_id)? {
                vertex.update(library, errors)?;
            }
        }

        Ok(())
    }

    /// Instantiate edges.
    pub fn instantiate_edges<StoreT, ErrorRecipientT>(
        &self,
        directory: &Directory,
        library: &mut Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorRecipientT: ErrorRecipient<FloriaError>,
    {
        for contained_vertex_id in &self.contained_vertex_ids {
            match library.store.get_vertex(contained_vertex_id)? {
                Some(contained_vertex) => {
                    contained_vertex.instantiate_edges(directory, library, errors)?;
                }

                None => tracing::warn!("vertex not found: {}", contained_vertex_id),
            }
        }

        let mut vertex = self.clone();

        match &vertex.instance.origin_template_id {
            Some(origin_template_id) => match library.store.get_vertex_template(origin_template_id)? {
                Some(vertex_template) => {
                    for outgoing_edge_template_id in &vertex_template.outgoing_edge_template_ids {
                        match library.store.get_edge_template(outgoing_edge_template_id)? {
                            Some(outgoing_edge_template) => {
                                match outgoing_edge_template.target_selector.select(
                                    &vertex.instance.id,
                                    outgoing_edge_template_id,
                                    library,
                                    errors,
                                )? {
                                    Some(target_vertex_id) => {
                                        let outgoing_edge_id = outgoing_edge_template.instantiate(
                                            directory,
                                            vertex.instance.id.clone(),
                                            target_vertex_id,
                                            &library.store,
                                        )?;

                                        vertex.outgoing_edge_ids.push(outgoing_edge_id);
                                    }

                                    None => {
                                        errors.give(FloriaError::Instantiation("target vertex not found".into()))?
                                    }
                                }
                            }

                            None => {
                                tracing::warn!("edge template not found: {}", outgoing_edge_template_id)
                            }
                        }
                    }
                }

                None => tracing::warn!("vertex template not found: {}", origin_template_id),
            },

            None => {}
        }

        library.store.add_vertex(vertex)?;

        Ok(())
    }
}
