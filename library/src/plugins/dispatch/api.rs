use super::{
    super::{
        super::{data::*, store::*},
        bindings::exports::floria::plugins::dispatch::CallSite,
        errors::*,
    },
    error::*,
    plugin::*,
};

use problemo::{common::*, *};

impl<StoreT> DispatchPlugin<StoreT>
where
    StoreT: Store,
{
    /// Initialize.
    pub fn initialize(&mut self) -> Result<(), Problem> {
        self.bindings
            .floria_plugins_dispatch()
            .call_initialize(&mut self.host, &self.id.to_string())
            .into_wasm_call_problem("initialize")?
            .gloss()
            .via(PluginError)
    }

    /// Dispatch.
    pub fn dispatch(
        &mut self,
        function: &str,
        arguments: Vec<Expression>,
        call_site: &CallSite,
    ) -> Result<Option<Expression>, Problem> {
        let mut dispatch_arguments = Vec::with_capacity(arguments.len());
        for argument in arguments.iter() {
            dispatch_arguments.push(self.expression_to_bindings(argument.clone())?);
        }

        // We create this just for tracing and errors
        let call = Call::new(self.id.clone(), function.into(), arguments, Default::default())?;

        tracing::debug!("dispatch: {} at {}", call, call_site);

        let expression = self
            .bindings
            .floria_plugins_dispatch()
            .call_dispatch(&mut self.host, function, &dispatch_arguments, call_site)
            .into_wasm_call_problem("dispatch")?
            .map_err(|error_depiction_markup| {
                DispatchError::as_problem(error_depiction_markup, call, call_site.clone())
            })
            .via(PluginError)?;

        Ok(match expression {
            Some(expression) => Some(self.expression_from_bindings(expression)?),
            None => None,
        })
    }
}
