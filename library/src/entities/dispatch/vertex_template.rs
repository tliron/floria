use super::{
    super::{
        super::{data::*, errors::*, plugins::*, store::*},
        vertex::*,
        vertex_template::*,
    },
    events::*,
};

use kutil::std::error::*;

impl VertexTemplate {
    /// Instantiate.
    pub fn instantiate<StoreT, ErrorReceiverT>(
        &self,
        directory: &Directory,
        containing_vertex_id: Option<ID>,
        payload: Option<&Expression>,
        context: &mut PluginContext<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<Vertex, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        let vertex_id = self.instantiate_vertexes(directory, containing_vertex_id, payload, context, errors)?;
        let vertex = context.store.get_vertex(&vertex_id)?.ok_or_else(|| StoreError::ID(vertex_id.to_string()))?;

        vertex.instantiate_edges(directory, context, errors)?;

        Ok(vertex)
    }

    /// Instantiate vertexes.
    pub fn instantiate_vertexes<StoreT, ErrorReceiverT>(
        &self,
        directory: &Directory,
        containing_vertex_id: Option<ID>,
        payload: Option<&Expression>,
        context: &mut PluginContext<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<ID, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        self.template.handle_instantiation_event(BEFORE_INSTANTIATION_EVENT, payload, context, errors)?;

        let mut vertex = Vertex::new_from_template(directory, self, containing_vertex_id, context.store.clone())?;
        let vertex_id = vertex.instance.id.clone();

        for contained_vertex_template_id in &self.contained_vertex_template_ids {
            match context.store.get_vertex_template(contained_vertex_template_id)? {
                Some(contained_vertex_template) => {
                    let contained_vertex_id = contained_vertex_template.instantiate_vertexes(
                        directory,
                        Some(vertex_id.clone()),
                        payload,
                        context,
                        errors,
                    )?;
                    vertex.contained_vertex_ids.push(contained_vertex_id);
                }

                None => tracing::warn!("vertex template not found: {}", contained_vertex_template_id),
            }
        }

        let event_handlers = vertex.instance.event_handlers.clone();

        context.store.add_vertex(vertex)?;

        event_handlers.handle_event(ADDED_EVENT, &vertex_id, payload, context, errors)?;

        Ok(vertex_id)
    }
}
