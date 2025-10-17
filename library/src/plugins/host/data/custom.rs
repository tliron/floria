use super::super::{
    super::{super::store::*, bindings::floria::plugins::floria as bindings},
    host::*,
};

use wasmtime::component::*;

//
// Custom
//

/// Custom.
#[derive(Clone, Debug)]
pub struct Custom {
    /// Kind.
    pub kind: String,

    /// Inner.
    pub inner: bindings::Expression,
}

impl Custom {
    /// Constructor.
    pub fn new(kind: String, inner: bindings::Expression) -> Self {
        Self { kind, inner }
    }
}

impl<StoreT> bindings::HostCustomResource for PluginHost<StoreT>
where
    StoreT: Store,
{
    fn new(&mut self, kind: String, inner: bindings::Expression) -> wasmtime::Result<Resource<Custom>> {
        Ok(self.resources.push(Custom::new(kind, inner))?)
    }

    fn drop(&mut self, resource: Resource<Custom>) -> wasmtime::Result<()> {
        self.resources.delete(resource)?;
        Ok(())
    }

    fn inner(&mut self, resource: Resource<Custom>) -> wasmtime::Result<(String, bindings::Expression)> {
        let custom = self.resources.get(&resource)?;
        Ok((custom.kind.clone(), custom.inner.clone()))
    }
}
