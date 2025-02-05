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
    pub fn dispatch<StoreT, ErrorRecipientT>(
        &self,
        call_site: &plugins::CallSite,
        library: &mut plugins::Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<Expression>, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorRecipientT: ErrorRecipient<FloriaError>,
    {
        tracing::debug!("call: {}", self);

        let mut arguments = Vec::with_capacity(self.arguments.len());
        for argument in &self.arguments {
            let argument = argument.evaluate(call_site, library, errors)?.unwrap_or_default();
            arguments.push(argument);
        }

        let plugin = library.dispatch_plugin(&self.plugin)?;
        let mut plugin = plugin.lock().map_err(plugins::PluginError::from)?;

        Ok(match plugin.dispatch(&self.function, arguments, call_site) {
            Ok(value) => value,
            Err(error) => {
                errors.give(error)?;
                None
            }
        })
    }
}
