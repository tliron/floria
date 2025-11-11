use super::super::{
    super::{data::*, store::*},
    bindings,
    context::*,
    errors::*,
    host::*,
};

use {problemo::*, std::path, wasmtime::component::*};

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
    pub fn new_from_component(component: Component, id: ID, context: &PluginContext<StoreT>) -> Result<Self, Problem> {
        // Host
        let mut host = wasmtime::Store::new(&context.environment.engine, PluginHost::new(id.clone(), context.clone()));

        // Linker

        let mut linker = Linker::new(&context.environment.engine);

        wasmtime_wasi::p2::add_to_linker_sync(&mut linker).into_wasm_link_problem("WASI bindings")?;

        #[cfg(feature = "plugins-http")]
        wasmtime_wasi_http::add_only_http_to_linker_sync(&mut linker).into_wasm_link_problem("WASI HTTP bindings")?;

        #[cfg(feature = "plugins-tls")]
        {
            let mut options = wasmtime_wasi_tls::LinkOptions::default();
            options.tls(true);
            wasmtime_wasi_tls::add_to_linker(&mut linker, &mut options, |host: &mut PluginHost<StoreT>| host.new_tls())
                .into_wasm_link_problem("WASI TLS bindings")?;
        }

        bindings::DispatchPlugin::add_to_linker::<_, HasSelf<_>>(&mut linker, |state: &mut PluginHost<StoreT>| state)
            .into_wasm_link_problem("dispatch plugin bindings")?;

        // Bindings
        let bindings = bindings::DispatchPlugin::instantiate(&mut host, &component, &linker)
            .into_wasm_link_problem("dispatch plugin bindings")?;

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
    ) -> Result<Self, Problem> {
        let component = if precompiled {
            // SAFETY: we are assuming the precompiled .cwasm is safe!
            unsafe {
                Component::deserialize(&context.environment.engine, bytes)
                    .into_wasm_load_problem("precompiled Wasm component from bytes")?
            }
        } else {
            Component::from_binary(&context.environment.engine, bytes)
                .into_wasm_load_problem("Wasm component from bytes")?
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
    ) -> Result<Self, Problem>
    where
        PathT: AsRef<path::Path>,
    {
        let component = if precompiled {
            // SAFETY: we are assuming the precompiled .cwasm is safe!
            unsafe {
                Component::deserialize_file(&context.environment.engine, path)
                    .into_wasm_load_problem("precompiled Wasm component from file")?
            }
        } else {
            Component::from_file(&context.environment.engine, path)
                .into_wasm_load_problem("Wasm component from file")?
        };

        Self::new_from_component(component, id, context)
    }
}
