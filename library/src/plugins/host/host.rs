use super::super::{
    super::{data::*, store::*},
    library::*,
};

use wasmtime_wasi::*;

//
// PluginHost
//

/// Floria plugin host.
pub struct PluginHost<StoreT>
where
    StoreT: 'static + Store,
{
    /// Plugin ID.
    pub id: ID,

    /// Library.
    pub library: Library<StoreT>,

    /// WASI context.
    pub wasi: WasiCtx,

    /// Resources.
    pub resources: ResourceTable,
}

impl<StoreT> PluginHost<StoreT>
where
    StoreT: Store,
{
    /// Constructor.
    pub fn new(id: ID, library: Library<StoreT>) -> Self {
        Self {
            id,
            library,
            wasi: WasiCtxBuilder::new().inherit_stdout().inherit_stderr().build(),
            resources: ResourceTable::default(),
        }
    }
}

impl<StoreT> WasiView for PluginHost<StoreT>
where
    StoreT: 'static + Send + Store,
{
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView { ctx: &mut self.wasi, table: &mut self.resources }
    }
}
