use super::{
    super::{
        super::{data::*, errors::*, plugins::*, store::*},
        instance::*,
    },
    events::*,
};

use kutil::std::error::*;

impl Instance {
    /// Handle event.
    pub fn handle_event<StoreT, ErrorReceiverT>(
        &self,
        event: &str,
        payload: Option<&Expression>,
        context: &mut PluginContext<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<Option<Expression>, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        self.event_handlers.handle_event(event, &self.id, payload, context, errors)
    }

    /// Prepare properties. Returns true if any property was modified.
    pub fn prepare_properties<StoreT, ErrorReceiverT>(
        &mut self,
        context: &mut PluginContext<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<bool, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        let mut modified = false;

        for (property_name, property) in self.properties.iter_mut() {
            if property.prepare(&self.id, &property_name, context, errors)? {
                modified = true;
            }
        }

        Ok(modified)
    }

    /// Update properties. Returns true if any property was modified.
    pub fn update_properties<StoreT, ErrorReceiverT>(
        &mut self,
        context: &mut PluginContext<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<bool, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        let mut modified = false;

        for (property_name, property) in self.properties.iter_mut() {
            if property.update(&self.id, &property_name, context, errors)? {
                modified = true;
            }
        }

        Ok(modified)
    }
}
