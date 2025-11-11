use super::{
    super::super::{
        super::{errors::*, plugins, store::*},
        expression::*,
    },
    call::*,
};

use kutil::std::error::*;

impl Call {
    /// Dispatch.
    pub fn dispatch<StoreT, ErrorReceiverT>(
        self,
        call_site: &plugins::CallSite,
        context: &mut plugins::PluginContext<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<Option<Expression>, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        tracing::debug!("call: {}", self);

        let mut arguments = Vec::with_capacity(self.arguments.len());
        for argument in self.arguments {
            let argument = argument.evaluate(call_site, context, errors)?.unwrap_or_default();
            arguments.push(argument);
        }

        let plugin_ref = must_unwrap_give!(context.maybe_load_dispatch_plugin_ref(&self.function.plugin_id), errors);
        let mut plugin = must_unwrap_give!(plugin_ref.lock().map_err(plugins::PluginError::from), errors);
        Ok(unwrap_or_give!(plugin.dispatch(&self.function.name, arguments, call_site), errors, None))
    }
}
