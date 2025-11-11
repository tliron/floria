use super::super::{
    super::{data::*, errors::*, plugins::*, store::*},
    edge::*,
    propagation::*,
};

use kutil::std::error::*;

impl Edge {
    /// Handle event.
    pub fn handle_event<StoreT, ErrorReceiverT>(
        &mut self,
        event: &str,
        arguments: &Vec<Expression>,
        propagation: &mut Propagation,
        context: &mut PluginContext<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<(), FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        self.instance.handle_event(event, arguments.clone(), context, errors)?;

        if propagation.source_vertex {
            if propagation.should(&self.source_vertex_id) {
                if let Some(mut source_vertex) = context.store.get_vertex(&self.source_vertex_id)? {
                    source_vertex.instance.handle_event(event, arguments.clone(), context, errors)?;
                }
            }
        }

        if propagation.target_vertex {
            if propagation.should(&self.target_vertex_id) {
                if let Some(mut target_vertex) = context.store.get_vertex(&self.target_vertex_id)? {
                    target_vertex.instance.handle_event(event, arguments.clone(), context, errors)?;
                }
            }
        }

        Ok(())
    }

    /// Update properties.
    pub fn update_properties<StoreT, ErrorReceiverT>(
        &mut self,
        propagation: &mut Propagation,
        context: &mut PluginContext<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<(), FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        if self.instance.update_properties(context, errors)? {
            context.store.add_edge(self.clone())?;
        }

        if self.instance.prepare_properties(context, errors)? {
            context.store.add_edge(self.clone())?;
        }

        if propagation.source_vertex {
            if propagation.should(&self.source_vertex_id) {
                if let Some(mut source_vertex) = context.store.get_vertex(&self.source_vertex_id)? {
                    source_vertex.update_properties(propagation, context, errors)?;
                }
            }
        }

        if propagation.target_vertex {
            if propagation.should(&self.target_vertex_id) {
                if let Some(mut target_vertex) = context.store.get_vertex(&self.target_vertex_id)? {
                    target_vertex.update_properties(propagation, context, errors)?;
                }
            }
        }

        Ok(())
    }
}
