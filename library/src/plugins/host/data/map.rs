use super::super::{
    super::{super::store::*, bindings::floria::plugins::floria as bindings},
    host::*,
};

use wasmtime::component::*;

//
// Map
//

/// Map.
#[derive(Clone, Debug, Default)]
pub struct Map {
    /// Inner.
    pub inner: Vec<(bindings::Expression, bindings::Expression)>,
}

impl Map {
    /// Constructor.
    pub fn new(inner: Vec<(bindings::Expression, bindings::Expression)>) -> Self {
        Self { inner }
    }
}

impl<StoreT> bindings::HostMapResource for PluginHost<StoreT>
where
    StoreT: Store,
{
    fn new(&mut self, inner: Vec<(bindings::Expression, bindings::Expression)>) -> wasmtime::Result<Resource<Map>> {
        Ok(self.resources.push(Map::new(inner))?)
    }

    fn drop(&mut self, resource: Resource<Map>) -> wasmtime::Result<()> {
        self.resources.delete(resource)?;
        Ok(())
    }

    fn get(&mut self, resource: Resource<Map>) -> wasmtime::Result<Vec<(bindings::Expression, bindings::Expression)>> {
        let map = self.resources.get(&resource)?;
        Ok(map.inner.clone().into_iter().collect())
    }

    fn length(&mut self, resource: Resource<Map>) -> wasmtime::Result<u64> {
        let map = self.resources.get(&resource)?;
        Ok((map.inner.len() / 2) as u64)
    }
}
