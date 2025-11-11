use super::super::{
    super::{data::*, store::*},
    bindings,
    context::*,
    errors::*,
    host::*,
};

use {anyhow::Context, std::path, wasmtime::component::*};

//
// DispatchPlugin
//

/// Floria dispatch plugin.
pub struct DispatchPlugin<StoreT>
where
    StoreT: 'static + Store,
{
    /// Plugin ID.
    pub id: ID,

    /// Host
    pub host: wasmtime::Store<PluginHost<StoreT>>,

    /// Bindings.
    pub bindings: bindings::DispatchPlugin,
}

impl<StoreT> DispatchPlugin<StoreT>
where
    StoreT: Clone + Send + Store,
{
    /// Constructor.
    pub fn new(id: ID, host: wasmtime::Store<PluginHost<StoreT>>, bindings: bindings::DispatchPlugin) -> Self {
        Self { id, host, bindings }
    }

    /// Constructor.
    pub fn new_from_component(
        component: Component,
        id: ID,
        context: &PluginContext<StoreT>,
    ) -> Result<Self, PluginError> {
        // Host
        let mut host = wasmtime::Store::new(&context.environment.engine, PluginHost::new(id.clone(), context.clone()));

        // Linker

        let mut linker = Linker::new(&context.environment.engine);

        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)
            .context("adding WASI bindings")
            .map_err(PluginError::LinkWasm)?;

        #[cfg(feature = "plugins-http")]
        wasmtime_wasi_http::add_only_http_to_linker_sync(&mut linker)
            .context("adding WASI HTTP bindings")
            .map_err(PluginError::LinkWasm)?;

        #[cfg(feature = "plugins-tls")]
        {
            let mut options = wasmtime_wasi_tls::LinkOptions::default();
            options.tls(true);
            wasmtime_wasi_tls::add_to_linker(&mut linker, &mut options, |host: &mut PluginHost<StoreT>| host.new_tls())
                .context("adding WASI TLS bindings")
                .map_err(PluginError::LinkWasm)?;
        }

        bindings::DispatchPlugin::add_to_linker::<_, HasSelf<_>>(&mut linker, |state: &mut PluginHost<StoreT>| state)
            .context("adding dispatch plugin bindings")
            .map_err(PluginError::LinkWasm)?;

        // Bindings
        let bindings = bindings::DispatchPlugin::instantiate(&mut host, &component, &linker)
            .context("instantiating dispatch plugin bindings")
            .map_err(PluginError::InstantiateWasm)?;

        Ok(Self::new(id, host, bindings))
    }

    /// Constructor.
    ///
    /// Prefer [new_from_file](Self::new_from_file) when a path is available, as the underlying
    /// implementation is optimized for this use case (by memory mapping the file).
    ///
    /// Make sure you trust the component when precompiled is true!
    pub fn new_from_bytes(
        bytes: &[u8],
        precompiled: bool,
        id: ID,
        context: &PluginContext<StoreT>,
    ) -> Result<Self, PluginError> {
        let component = if precompiled {
            // SAFETY: we are assuming the precompiled .cwasm is safe!
            unsafe {
                Component::deserialize(&context.environment.engine, bytes)
                    .context("loading precompiled Wasm component from bytes")
                    .map_err(PluginError::LoadWasm)?
            }
        } else {
            Component::from_binary(&context.environment.engine, bytes)
                .context("loading Wasm component from bytes")
                .map_err(PluginError::LoadWasm)?
        };

        Self::new_from_component(component, id, context)
    }

    /// Constructor.
    ///
    /// Make sure you trust the component when precompiled is true!
    pub fn new_from_file<PathT>(
        path: PathT,
        precompiled: bool,
        id: ID,
        context: &PluginContext<StoreT>,
    ) -> Result<Self, PluginError>
    where
        PathT: AsRef<path::Path>,
    {
        let component = if precompiled {
            // SAFETY: we are assuming the precompiled .cwasm is safe!
            unsafe {
                Component::deserialize_file(&context.environment.engine, path)
                    .context("loading precompiled Wasm component from file")
                    .map_err(PluginError::LoadWasm)?
            }
        } else {
            Component::from_file(&context.environment.engine, path)
                .context("loading Wasm component from file")
                .map_err(PluginError::LoadWasm)?
        };

        Self::new_from_component(component, id, context)
    }
}
