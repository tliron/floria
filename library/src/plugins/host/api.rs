use super::{
    super::{super::store::*, bindings::floria::plugins::floria as bindings},
    host::*,
};

use kutil::std::error::*;

impl<StoreT> bindings::Host for PluginHost<StoreT>
where
    StoreT: Clone + Send + Store,
{
    fn log(&mut self, source: String, message: String) -> wasmtime::Result<()> {
        tracing::info!("[{}] {}: {}", self.name, source, message);
        Ok(())
    }

    fn evaluate_expression(
        &mut self,
        expression: bindings::Expression,
        call_site: bindings::CallSite,
    ) -> wasmtime::Result<Result<Option<bindings::Expression>, String>> {
        // TODO: also need to make sure we're not calling into same plugin
        let expression = self.expression_from_bindings(expression)?;

        Ok(Ok(match expression.evaluate(&call_site.into(), &mut self.library, &mut FailFastErrorRecipient)? {
            Some(value) => Some(self.expression_to_bindings(value)?),
            None => None,
        }))
    }

    fn get_entity(&mut self, id: bindings::Id) -> wasmtime::Result<Result<bindings::Expression, String>> {
        Ok(match self.library.store.get_entity_as_expression(&id.into())? {
            Some(entity) => Ok(self.expression_to_bindings(entity)?),
            None => todo!(),
        })
    }
}
