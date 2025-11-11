use super::super::{
    super::{
        data::{Call, *},
        errors::*,
        plugins::*,
        store::*,
    },
    events::*,
};

use kutil::std::error::*;

/// Update event.
pub const UPDATE_EVENT: &str = "floria:update";

/// Added event.
pub const ADDED_EVENT: &str = "floria:added";

/// Before instantiation event.
pub const BEFORE_INSTANTIATION_EVENT: &str = "floria:instantiation:before";

//
// HandleEvent
//

/// Handle event.
pub trait HandleEvent {
    /// Handle event.
    fn handle_event<StoreT, ErrorReceiverT>(
        &self,
        event: &str,
        id: &ID,
        payload: Option<&Expression>,
        context: &mut PluginContext<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<Option<Expression>, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>;
}

impl HandleEvent for EventHandlers {
    fn handle_event<StoreT, ErrorReceiverT>(
        &self,
        event: &str,
        id: &ID,
        payload: Option<&Expression>,
        context: &mut PluginContext<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<Option<Expression>, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        if let Some(event_handlers) = self.get(event) {
            let call_site = CallSite::new(id.clone(), None);
            let arguments = match payload {
                Some(payload) => vec![payload.clone()],
                None => Default::default(),
            };

            for event_handler in event_handlers {
                let call =
                    Call::new(event_handler.plugin_id.clone(), event_handler.name.clone(), arguments, CallKind::Eager)?;
                // TODO: all handlers
                return call.dispatch(&call_site, context, errors);
            }
        };

        Ok(None)
    }
}
