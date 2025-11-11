use super::{
    super::super::{
        super::{plugins, store::*},
        expression::*,
    },
    call::*,
};

use problemo::{common::*, *};

impl Call {
    /// Dispatch.
    pub fn dispatch<StoreT, ProblemReceiverT>(
        self,
        call_site: &plugins::CallSite,
        context: &mut plugins::PluginContext<StoreT>,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<Expression>, Problem>
    where
        StoreT: Clone + Send + Store,
        ProblemReceiverT: ProblemReceiver,
    {
        tracing::debug!("call: {}", self);

        let mut arguments = Vec::with_capacity(self.arguments.len());
        for argument in self.arguments {
            let argument = argument.evaluate(call_site, context, problems)?.unwrap_or_default();
            arguments.push(argument);
        }

        let plugin_ref = give_unwrap!(context.maybe_load_dispatch_plugin_ref(&self.function.plugin_id), problems);
        let mut plugin = give_unwrap!(plugin_ref.lock().into_thread_problem(), problems);
        Ok(give_unwrap!(plugin.dispatch(&self.function.name, arguments, call_site), problems, None))
    }
}
