use super::{
    super::{
        super::store::*,
        bindings::{self, exports::floria::plugins::dispatch::Site},
        errors::*,
        host::*,
        library::*,
    },
    error::*,
};

use {compris::normal::*, kutil::std::immutable::*, std::path, wasmtime::component::*};

//
// DispatchPlugin
//

/// Floria dispatch plugin.
pub struct DispatchPlugin<StoreT>
where
    StoreT: 'static + Store,
{
    /// Name.
    pub name: ByteString,

    pub(crate) host: wasmtime::Store<PluginHost<StoreT>>,
    pub(crate) bindings: bindings::DispatchPlugin,
}

impl<StoreT> DispatchPlugin<StoreT>
where
    StoreT: Clone + Send + Store,
{
    /// Constructor.
    pub fn new_from_bytes(bytes: &[u8], name: ByteString, library: &Library<StoreT>) -> Result<Self, PluginError> {
        let component = Component::from_binary(&library.environment.engine, bytes).map_err(PluginError::LoadWasm)?;
        Self::new_from_component(component, name, library)
    }

    /// Constructor.
    pub fn new_from_component(
        component: Component,
        name: ByteString,
        library: &Library<StoreT>,
    ) -> Result<Self, PluginError> {
        // Linker
        let mut linker = Linker::new(&library.environment.engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker).map_err(PluginError::LinkWasm)?;
        bindings::DispatchPlugin::add_to_linker::<_, HasSelf<_>>(&mut linker, |state: &mut PluginHost<StoreT>| state)
            .map_err(PluginError::LinkWasm)?;

        // Host
        let mut host =
            wasmtime::Store::new(&library.environment.engine, PluginHost::new(name.clone(), library.clone()));

        // Bindings
        let bindings = bindings::DispatchPlugin::instantiate(&mut host, &component, &linker)
            .map_err(PluginError::InstantiateWasm)?;

        Ok(Self { name, host, bindings })
    }

    /// Constructor.
    pub fn new_from_file<PathT>(path: PathT, name: ByteString, library: &Library<StoreT>) -> Result<Self, PluginError>
    where
        PathT: AsRef<path::Path>,
    {
        let component = Component::from_file(&library.environment.engine, path).map_err(PluginError::LoadWasm)?;
        Self::new_from_component(component, name, library)
    }

    /// Dispatch.
    pub fn dispatch<AnnotatedT>(
        &mut self,
        name: &str,
        arguments: Vec<Variant<AnnotatedT>>,
        site: &Site,
    ) -> Result<Variant<AnnotatedT>, PluginError>
    where
        AnnotatedT: Default,
    {
        let length = arguments.len();
        let mut string_arguments = Vec::with_capacity(length);
        let mut dispatch_arguments = Vec::with_capacity(length);
        for argument in arguments.into_iter() {
            string_arguments.push(argument.to_string());
            dispatch_arguments.push(self.to_guest_any(argument)?);
        }

        tracing::debug!("dispatch: {}({}) at {}", name, string_arguments.join(","), site);

        let value = self
            .bindings
            .floria_plugins_dispatch()
            .call_dispatch(&mut self.host, name, &dispatch_arguments, site)
            .map_err(PluginError::CallWasm)?
            .map_err(|error| {
                DispatchError::new(
                    error.to_string(),
                    self.name.to_string(),
                    name.into(),
                    string_arguments,
                    site.clone(),
                )
            })?;

        Ok(self.from_guest_value(value)?)
    }
}
