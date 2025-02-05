use super::super::{super::super::store::*, super::bindings::floria::plugins::floria as bindings, host::*};

use wasmtime::component::*;

//
// Call
//

/// Call.
#[derive(Clone, Debug)]
pub struct Call {
    /// Plugin name.
    pub plugin: String,

    /// Function name.
    pub function: String,

    /// Arguments.
    pub arguments: Vec<bindings::Expression>,

    /// Kind.
    pub kind: bindings::CallKind,
}

impl Call {
    /// Constructor.
    pub fn new(
        plugin: String,
        function: String,
        arguments: Vec<bindings::Expression>,
        kind: bindings::CallKind,
    ) -> Self {
        Self { plugin, function, arguments, kind }
    }
}

impl<StoreT> bindings::HostCallResource for PluginHost<StoreT>
where
    StoreT: Store,
{
    fn new(
        &mut self,
        plugin: String,
        function: String,
        arguments: Vec<bindings::Expression>,
        kind: bindings::CallKind,
    ) -> wasmtime::Result<Resource<Call>> {
        let call = Call::new(plugin, function, arguments, kind);
        Ok(self.resources.push(call)?)
    }

    fn drop(&mut self, resource: Resource<Call>) -> wasmtime::Result<()> {
        self.resources.delete(resource)?;
        Ok(())
    }

    fn get(
        &mut self,
        resource: Resource<Call>,
    ) -> wasmtime::Result<(String, String, Vec<bindings::Expression>, bindings::CallKind)> {
        let call = self.resources.get(&resource)?;
        Ok((call.plugin.clone(), call.function.clone(), call.arguments.clone(), call.kind))
    }
}
