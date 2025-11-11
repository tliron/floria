use super::{
    super::{
        super::{data::*, plugins::*, store::*},
        template::*,
    },
    events::*,
};

use problemo::*;

impl Template {
    /// Handle instantiation event.
    pub fn handle_instantiation_event<StoreT, ProblemReceiverT>(
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
        self.instantiation_event_handlers.handle_event(event, &self.id, payload, context, problems)
    }
}
