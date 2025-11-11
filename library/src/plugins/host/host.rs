use super::super::{
    super::{data::*, store::*},
    context::*,
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

    /// Plugin context.
    pub context: PluginContext<StoreT>,

    /// WASI context.
    pub wasi: WasiCtx,

    /// WASI HTTP context.
    #[cfg(feature = "plugins-http")]
    pub wasi_http: wasmtime_wasi_http::WasiHttpCtx,

    /// WASI TLS context.
    #[cfg(feature = "plugins-tls")]
    pub wasi_tls: wasmtime_wasi_tls::WasiTlsCtx,

    /// Resources.
    pub resources: ResourceTable,
}

impl<StoreT> PluginHost<StoreT>
where
    StoreT: Store,
{
    /// Constructor.
    pub fn new(id: ID, context: PluginContext<StoreT>) -> Self {
        Self {
            id,
            context,
            wasi: WasiCtxBuilder::new()
                .inherit_stdout()
                .inherit_stderr()
                .inherit_network()
                .allow_ip_name_lookup(true)
                .build(),
            #[cfg(feature = "plugins-http")]
            wasi_http: wasmtime_wasi_http::WasiHttpCtx::new(),
            #[cfg(feature = "plugins-tls")]
            wasi_tls: wasmtime_wasi_tls::WasiTlsCtxBuilder::new().build(),
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
