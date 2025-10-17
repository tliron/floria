use super::super::{super::super::store::*, super::bindings::floria::plugins::floria as bindings, host::*};

use wasmtime::component::*;

//
// List
//

/// List.
#[derive(Clone, Debug, Default)]
pub struct List {
    /// List.
    pub list: Vec<bindings::Expression>,
}

impl List {
    /// Constructor.
    pub fn new(list: Vec<bindings::Expression>) -> Self {
        Self { list }
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

    fn inner(&mut self, resource: Resource<List>) -> wasmtime::Result<Vec<bindings::Expression>> {
        let list = self.resources.get(&resource)?;
        Ok(list.list.clone())
    }

    fn length(&mut self, resource: Resource<List>) -> wasmtime::Result<u64> {
        let list = self.resources.get(&resource)?;
        Ok(list.list.len() as u64)
    }
}
