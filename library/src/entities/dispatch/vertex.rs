use super::super::{
    super::{data::*, plugins::*, store::*},
    events::*,
    propagation::*,
    vertex::*,
};

use problemo::{common::*, *};

impl Vertex {
    /// Handle event.
    pub fn handle_event<StoreT, ProblemReceiverT>(
        &mut self,
        event: &str,
        payload: Option<&Expression>,
        propagation: &mut Propagation,
        context: &mut PluginContext<StoreT>,
        problems: &mut ProblemReceiverT,
    ) -> Result<(), Problem>
    where
        StoreT: Clone + Send + Store,
        ProblemReceiverT: ProblemReceiver,
    {
        match event {
            UPDATE_EVENT => {
                let property_names: Vec<_> = self.instance.properties.keys().cloned().collect();

                for property_name in &property_names {
                    if let Some(property) = self.instance.properties.get_mut(property_name)
                        && property.update(&self.instance.id, &property_name, context, problems)?
                    {
                        context.store.add_vertex(self.clone())?;
                    }

                    if let Some(property) = self.instance.properties.get_mut(property_name)
                        && property.prepare(&self.instance.id, &property_name, context, problems)?
                    {
                        context.store.add_vertex(self.clone())?;
                    }
                }
            }

            _ => {
                self.instance.handle_event(event, payload, context, problems)?;
            }
        }

        if propagation.containing_vertex
            && let Some(containing_vertex_id) = &self.containing_vertex_id
        {
            if propagation.should(containing_vertex_id) {
                if let Some(mut containing_vertex) = context.store.get_vertex(containing_vertex_id)? {
                    containing_vertex.handle_event(event, payload, propagation, context, problems)?;
                }
            }
        }

        if propagation.contained_vertexes {
            for contained_vertex_id in &self.contained_vertex_ids {
                if propagation.should(contained_vertex_id) {
                    if let Some(mut contained_vertex) = context.store.get_vertex(contained_vertex_id)? {
                        contained_vertex.handle_event(event, payload, propagation, context, problems)?;
                    }
                }
            }
        }

        if propagation.incoming_edges {
            for incoming_edge_id in &self.incoming_edge_ids {
                if propagation.should(incoming_edge_id) {
                    if let Some(mut incoming_edge) = context.store.get_edge(incoming_edge_id)? {
                        incoming_edge.handle_event(event, payload, propagation, context, problems)?;
                    }
                }
            }
        }

        if propagation.outgoing_edges {
            for outgoing_edge_id in &self.outgoing_edge_ids {
                if propagation.should(outgoing_edge_id) {
                    if let Some(mut outgoing_edge) = context.store.get_edge(outgoing_edge_id)? {
                        outgoing_edge.handle_event(event, payload, propagation, context, problems)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Instantiate edges.
    pub fn instantiate_edges<StoreT, ProblemReceiverT>(
        &self,
        directory: &Directory,
        context: &mut PluginContext<StoreT>,
        problems: &mut ProblemReceiverT,
    ) -> Result<(), Problem>
    where
        StoreT: Clone + Send + Store,
        ProblemReceiverT: ProblemReceiver,
    {
        for contained_vertex_id in &self.contained_vertex_ids {
            match give_unwrap!(context.store.get_vertex(contained_vertex_id), problems) {
                Some(contained_vertex) => {
                    contained_vertex.instantiate_edges(directory, context, problems)?;
                }

                None => tracing::warn!("vertex not found: {}", contained_vertex_id),
            }
        }

        let mut vertex = self.clone();

        match &vertex.instance.origin_template_id {
            Some(origin_template_id) => {
                match give_unwrap!(context.store.get_vertex_template(origin_template_id), problems) {
                    Some(vertex_template) => {
                        for outgoing_edge_template_id in &vertex_template.outgoing_edge_template_ids {
                            match give_unwrap!(context.store.get_edge_template(outgoing_edge_template_id), problems) {
                                Some(outgoing_edge_template) => {
                                    match outgoing_edge_template.target_selector.select(
                                        &vertex.instance.id,
                                        outgoing_edge_template_id,
                                        context,
                                        problems,
                                    )? {
                                        Some(target_vertex_id) => {
                                            if let Some(outgoing_edge_id) = outgoing_edge_template
                                                .instantiate(
                                                    directory,
                                                    vertex.instance.id.clone(),
                                                    target_vertex_id,
                                                    context.store.clone(),
                                                )
                                                .give_ok(problems)?
                                            {
                                                vertex.outgoing_edge_ids.push(outgoing_edge_id);
                                            }
                                        }

                                        None => problems.give(MissingError::new("target vertex").into())?,
                                    }
                                }

                                None => {
                                    tracing::warn!("edge template not found: {}", outgoing_edge_template_id)
                                }
                            }
                        }
                    }

                    None => tracing::warn!("vertex template not found: {}", origin_template_id),
                }
            }

            None => {}
        }

        context.store.add_vertex(vertex)?;

        Ok(())
    }
}
