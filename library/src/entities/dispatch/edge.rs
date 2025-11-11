use super::{
    super::{
        super::{data::*, errors::*, plugins::*, store::*},
        edge::*,
        propagation::*,
    },
    events::*,
};

use kutil::std::error::*;

impl Edge {
    /// Handle event.
    pub fn handle_event<StoreT, ErrorReceiverT>(
        &mut self,
        event: &str,
        payload: Option<&Expression>,
        propagation: &mut Propagation,
        context: &mut PluginContext<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<(), FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        match event {
            UPDATE_EVENT => {
                let property_names: Vec<_> = self.instance.properties.keys().cloned().collect();

                for property_name in &property_names {
                    if let Some(property) = self.instance.properties.get_mut(property_name)
                        && property.update(&self.instance.id, &property_name, context, errors)?
                    {
                        context.store.add_edge(self.clone())?;
                    }

                    if let Some(property) = self.instance.properties.get_mut(property_name)
                        && property.prepare(&self.instance.id, &property_name, context, errors)?
                    {
                        context.store.add_edge(self.clone())?;
                    }
                }
            }

            _ => {
                self.instance.handle_event(event, payload, context, errors)?;
            }
        }

        if propagation.source_vertex {
            if propagation.should(&self.source_vertex_id) {
                if let Some(source_vertex) = context.store.get_vertex(&self.source_vertex_id)? {
                    source_vertex.instance.handle_event(event, payload, context, errors)?;
                }
            }
        }

        if propagation.target_vertex {
            if propagation.should(&self.target_vertex_id) {
                if let Some(target_vertex) = context.store.get_vertex(&self.target_vertex_id)? {
                    target_vertex.instance.handle_event(event, payload, context, errors)?;
                }
            }
        }

        Ok(())
    }
}
