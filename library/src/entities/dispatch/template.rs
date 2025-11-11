use super::{
    super::{
        super::{data::*, errors::*, plugins::*, store::*},
        template::*,
    },
    events::*,
};

use kutil::std::error::*;

impl Template {
    /// Handle instantiation event.
    pub fn handle_instantiation_event<StoreT, ErrorReceiverT>(
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
        self.instantiation_event_handlers.handle_event(event, &self.id, payload, context, errors)
    }
}
