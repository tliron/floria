use super::super::{
    super::{data::*, errors::*, plugins::*, store::*},
    vertex::*,
    vertex_template::*,
};

use kutil::std::error::*;

impl VertexTemplate {
    /// Instantiate.
    pub fn instantiate<StoreT, ErrorReceiverT>(
        &self,
        directory: &Directory,
        containing_vertex_id: Option<ID>,
        library: &mut Library<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<Vertex, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        let vertex_id = self.instantiate_vertexes(directory, containing_vertex_id, library, errors)?;
        let vertex = library.store.get_vertex(&vertex_id)?.ok_or_else(|| StoreError::ID(vertex_id.to_string()))?;

        vertex.instantiate_edges(directory, library, errors)?;

        Ok(vertex)
    }

    /// Instantiate vertexes.
    pub fn instantiate_vertexes<StoreT, ErrorReceiverT>(
        &self,
        directory: &Directory,
        containing_vertex_id: Option<ID>,
        library: &mut Library<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<ID, FloriaError>
    where
        StoreT: Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        let mut vertex = Vertex {
            instance: self.template.instantiate(EntityKind::Vertex, directory, &library.store)?,
            containing_vertex_id: containing_vertex_id,
            contained_vertex_ids: Vec::with_capacity(self.contained_vertex_template_ids.len()),
            outgoing_edge_ids: Default::default(),
            incoming_edge_ids: Default::default(),
        };

        let vertex_id = vertex.instance.id.clone();

        for contained_vertex_template_id in &self.contained_vertex_template_ids {
            match library.store.get_vertex_template(contained_vertex_template_id)? {
                Some(contained_vertex_template) => {
                    let contained_vertex_id = contained_vertex_template.instantiate_vertexes(
                        directory,
                        Some(vertex_id.clone()),
                        library,
                        errors,
                    )?;
                    vertex.contained_vertex_ids.push(contained_vertex_id);
                }

                None => tracing::warn!("vertex template not found: {}", contained_vertex_template_id),
            }
        }

        library.store.add_vertex(vertex)?;

        Ok(vertex_id)
    }
}
