use super::{
    super::{
        super::{super::store::*, bindings::floria::plugins::floria as bindings},
        host::*,
    },
    expression::*,
};

use {std::mem::*, wasmtime::component::*};

//
// Map
//

/// Map.
#[derive(Debug, Default)]
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
    fn new(
        &mut self,
        key_value_pairs: Vec<(bindings::Expression, bindings::Expression)>,
    ) -> wasmtime::Result<Resource<Map>> {
        Ok(self.resources.push(Map::new(key_value_pairs))?)
    }

    fn drop(&mut self, resource: Resource<Map>) -> wasmtime::Result<()> {
        self.resources.delete(resource)?;
        Ok(())
    }

    fn take(&mut self, resource: Resource<Map>) -> wasmtime::Result<Vec<(bindings::Expression, bindings::Expression)>> {
        Ok(take(&mut self.resources.get_mut(&resource)?.inner))
    }

    fn length(&mut self, resource: Resource<Map>) -> wasmtime::Result<u64> {
        let map = self.resources.get(&resource)?;
        Ok(map.inner.len() as u64)
    }

    fn get(
        &mut self,
        resource: Resource<Map>,
        key: bindings::Expression,
    ) -> wasmtime::Result<Option<bindings::Expression>> {
        let map = self.resources.get(&resource)?;
        for (key_, value) in &map.inner {
            if self.expression_equal(key_, &key)? {
                return Ok(Some(self.deep_clone_expression(value.shallow_clone())?));
            }
        }
        Ok(None)
    }
}
