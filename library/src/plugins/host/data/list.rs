use super::super::{super::super::store::*, super::bindings::floria::plugins::floria as bindings, host::*};

use wasmtime::component::*;

//
// List
//

/// List.
#[derive(Clone, Debug, Default)]
pub struct List {
    /// Inner.
    pub inner: Vec<bindings::Expression>,
}

impl List {
    /// Constructor.
    pub fn new(inner: Vec<bindings::Expression>) -> Self {
        Self { inner }
    }
}

impl<StoreT> bindings::HostListResource for PluginHost<StoreT>
where
    StoreT: Store,
{
    fn new(&mut self, list: Vec<bindings::Expression>) -> wasmtime::Result<Resource<List>> {
        Ok(self.resources.push(List::new(list))?)
    }

    fn drop(&mut self, resource: Resource<List>) -> wasmtime::Result<()> {
        self.resources.delete(resource)?;
        Ok(())
    }

    fn get(&mut self, resource: Resource<List>) -> wasmtime::Result<Vec<bindings::Expression>> {
        let list = self.resources.get(&resource)?;
        Ok(list.inner.clone())
    }

    fn length(&mut self, resource: Resource<List>) -> wasmtime::Result<u64> {
        let list = self.resources.get(&resource)?;
        Ok(list.inner.len() as u64)
    }
}
