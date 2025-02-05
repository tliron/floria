use super::{
    super::{
        super::{data::*, store::*},
        bindings::exports::floria::plugins::dispatch::CallSite,
        errors::*,
    },
    error::*,
    plugin::*,
};

impl<StoreT> DispatchPlugin<StoreT>
where
    StoreT: Store,
{
    /// Initialize.
    pub fn initialize(&mut self) -> Result<(), PluginError> {
        self.bindings
            .floria_plugins_dispatch()
            .call_initialize(&mut self.host)
            .map_err(PluginError::CallWasm)?
            .map_err(|error| InitializationError::new(error.to_string()).into())
    }

    /// Dispatch.
    pub fn dispatch(
        &mut self,
        name: &str,
        arguments: Vec<Expression>,
        call_site: &CallSite,
    ) -> Result<Option<Expression>, PluginError> {
        let mut dispatch_arguments = Vec::with_capacity(arguments.len());
        for argument in arguments.iter() {
            dispatch_arguments.push(self.expression_to_bindings(argument.clone())?);
        }

        // We create this just for tracing and errors
        let call = Call::new(self.name.to_string().into(), name.into(), arguments, Default::default());

        tracing::debug!("dispatch: {} at {}", call, call_site);

        let value = self
            .bindings
            .floria_plugins_dispatch()
            .call_dispatch(&mut self.host, name, &dispatch_arguments, call_site)
            .map_err(PluginError::CallWasm)?
            .map_err(|error| DispatchError::new(error.to_string(), call, call_site.clone()))?;

        Ok(match value {
            Some(value) => Some(self.expression_from_bindings(value)?),
            None => None,
        })
    }
}
