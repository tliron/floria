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
    /// Map.
    pub key_value_pairs: Vec<(bindings::Expression, bindings::Expression)>,
}

impl Map {
    /// Constructor.
    pub fn new(key_value_pairs: Vec<(bindings::Expression, bindings::Expression)>) -> Self {
        Self { key_value_pairs }
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

    fn inner(
        &mut self,
        resource: Resource<Map>,
    ) -> wasmtime::Result<Vec<(bindings::Expression, bindings::Expression)>> {
        let map = self.resources.get(&resource)?;
        Ok(map.key_value_pairs.clone())
    }

    fn length(&mut self, resource: Resource<Map>) -> wasmtime::Result<u64> {
        let map = self.resources.get(&resource)?;
        Ok(map.key_value_pairs.len() as u64)
    }
}
