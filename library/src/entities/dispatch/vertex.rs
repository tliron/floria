use super::super::{
    super::{data::*, errors::*, plugins::*, store::*},
    propagation::*,
    vertex::*,
};

use kutil::std::error::*;

impl Vertex {
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
            library.store.add_vertex(self.clone())?;
        }

        if self.instance.prepare_properties(library, errors)? {
            library.store.add_vertex(self.clone())?;
        }

        if propagation.containing_vertex
            && let Some(containing_vertex_id) = &self.containing_vertex_id
        {
            if propagation.should(containing_vertex_id) {
                if let Some(mut containing_vertex) = library.store.get_vertex(containing_vertex_id)? {
                    containing_vertex.update_properties(propagation, library, errors)?;
                }
            }
        }

        if propagation.contained_vertexes {
            for contained_vertex_id in &self.contained_vertex_ids {
                if propagation.should(contained_vertex_id) {
                    if let Some(mut contained_vertex) = library.store.get_vertex(contained_vertex_id)? {
                        contained_vertex.update_properties(propagation, library, errors)?;
                    }
                }
            }
        }

        if propagation.incoming_edges {
            for incoming_edge_id in &self.incoming_edge_ids {
                if propagation.should(incoming_edge_id) {
                    if let Some(mut incoming_edge) = library.store.get_edge(incoming_edge_id)? {
                        incoming_edge.update_properties(propagation, library, errors)?;
                    }
                }
            }
        }

        if propagation.outgoing_edges {
            for outgoing_edge_id in &self.outgoing_edge_ids {
                if propagation.should(outgoing_edge_id) {
                    if let Some(mut outgoing_edge) = library.store.get_edge(outgoing_edge_id)? {
                        outgoing_edge.update_properties(propagation, library, errors)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Instantiate edges.
    pub fn instantiate_edges<StoreT, ErrorReceiverT>(
        &self,
        directory: &Directory,
        library: &mut Library<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<(), FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
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
