use super::super::{
    super::{
        data::{Call, *},
        errors::*,
        plugins::*,
        store::*,
    },
    instance::*,
};

use kutil::std::error::*;

impl Instance {
    /// Handle event.
    pub fn handle_event<StoreT, ErrorReceiverT>(
        &mut self,
        event: &str,
        arguments: Vec<Expression>,
        library: &mut Library<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<Option<Expression>, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        if let Some(event_handler) = self.event_handlers.get(event) {
            let call_site = CallSite::new(self.id.clone(), None);
            let call =
                Call::new(event_handler.plugin_id.clone(), event_handler.name.clone(), arguments, CallKind::Eager)?;
            return call.dispatch(&call_site, library, errors);
        };

        Ok(None)
    }

    /// Prepare properties. Returns true if any property was modified.
    pub fn prepare_properties<StoreT, ErrorReceiverT>(
        &mut self,
        library: &mut Library<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<bool, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        let mut modified = false;

        for (property_name, property) in self.properties.iter_mut() {
            if property.prepare(&self.id, &property_name, library, errors)? {
                modified = true;
            }
        }

        Ok(modified)
    }

    /// Update properties. Returns true if any property was modified.
    pub fn update_properties<StoreT, ErrorReceiverT>(
        &mut self,
        library: &mut Library<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<bool, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        let mut modified = false;

        for (property_name, property) in self.properties.iter_mut() {
            if property.update(&self.id, &property_name, library, errors)? {
                modified = true;
            }
        }

        Ok(modified)
    }
}
