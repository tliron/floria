use super::super::{
    super::{
        data::{Call, *},
        plugins::*,
        store::*,
    },
    events::*,
};

use problemo::*;

//
// HandleEvent
//

/// Handle event.
pub trait HandleEvent {
    /// Handle event.
    fn handle_event<StoreT, ProblemReceiverT>(
        &self,
        event: &str,
        id: &ID,
        payload: Option<&Expression>,
        context: &mut PluginContext<StoreT>,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<Expression>, Problem>
    where
        StoreT: Clone + Send + Store,
        ProblemReceiverT: ProblemReceiver;
}

impl HandleEvent for EventHandlers {
    fn handle_event<StoreT, ProblemReceiverT>(
        &self,
        event: &str,
        id: &ID,
        payload: Option<&Expression>,
        context: &mut PluginContext<StoreT>,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<Expression>, Problem>
    where
        StoreT: Clone + Send + Store,
        ProblemReceiverT: ProblemReceiver,
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
                return call.dispatch(&call_site, context, problems);
            }
        };

        Ok(None)
    }
}
