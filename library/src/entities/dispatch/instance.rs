use super::{
    super::{
        super::{data::*, plugins::*, store::*},
        instance::*,
    },
    events::*,
};

use problemo::*;

impl Instance {
    /// Handle event.
    pub fn handle_event<StoreT, ProblemReceiverT>(
        &self,
        event: &str,
        payload: Option<&Expression>,
        context: &mut PluginContext<StoreT>,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<Expression>, Problem>
    where
        StoreT: Clone + Send + Store,
        ProblemReceiverT: ProblemReceiver,
    {
        self.event_handlers.handle_event(event, &self.id, payload, context, problems)
    }
}
