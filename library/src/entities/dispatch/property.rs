use super::super::{
    super::{data::*, plugins::*, store::*},
    property::*,
};

use problemo::*;

impl Property {
    /// Prepare. Returns true if modified.
    pub fn prepare<StoreT, ProblemReceiverT>(
        &mut self,
        id: &ID,
        property_name: &str,
        context: &mut PluginContext<StoreT>,
        problems: &mut ProblemReceiverT,
    ) -> Result<bool, Problem>
    where
        StoreT: Clone + Send + Store,
        ProblemReceiverT: ProblemReceiver,
    {
        if self.value.is_none() {
            return Ok(false);
        }

        if let Some(preparer) = &self.preparer {
            let call_site = CallSite::new(id.clone(), Some(property_name.into()));
            if let Some(expression) = preparer.clone().evaluate(&call_site, context, problems)? {
                self.value = Some(expression.into());
            }
            return Ok(true);
        }

        Ok(false)
    }

    /// Update. Returns true if modified.
    pub fn update<StoreT, ProblemReceiverT>(
        &mut self,
        id: &ID,
        property_name: &str,
        context: &mut PluginContext<StoreT>,
        problems: &mut ProblemReceiverT,
    ) -> Result<bool, Problem>
    where
        StoreT: Clone + Send + Store,
        ProblemReceiverT: ProblemReceiver,
    {
        if self.read_only && self.value.is_some() {
            // Read-only properties can only be updated once.
            return Ok(false);
        }

        if let Some(updater) = &self.updater {
            let call_site = CallSite::new(id.clone(), Some(property_name.into()));
            if let Some(expression) = updater.clone().evaluate(&call_site, context, problems)? {
                self.value = Some(expression.into());
            }
            return Ok(true);
        }

        Ok(false)
    }
}
