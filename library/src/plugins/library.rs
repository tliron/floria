use super::{super::store::*, dispatch::*, environment::*, errors::*};

use {
    kutil::std::{collections::*, immutable::*},
    std::{path, sync::*},
};

//
// Library
//

/// Plugin library.
///
/// Cloning is cheap and clones always refer to the same shared state.
#[derive(Clone)]
pub struct Library<StoreT>
where
    StoreT: 'static + Store,
{
    /// Environment.
    pub environment: Environment,

    /// Store.
    pub store: StoreT,

    /// Dispatch plugins.
    pub dispatch_plugins: Arc<FastConcurrentHashMap<ByteString, DispatchPluginRef<StoreT>>>,
}

impl<StoreT> Library<StoreT>
where
    StoreT: Store,
{
    /// Constructor.
    pub fn new(environment: Environment, store: StoreT) -> Self {
        Self { environment, store, dispatch_plugins: Default::default() }
    }

    /// Add a dispatch plugin.
    pub fn add_dispatch_plugin(
        &mut self,
        plugin_name: ByteString,
        bytes: &[u8],
        precompiled: bool,
    ) -> Result<(), PluginError>
    where
        StoreT: Clone + Send,
    {
        let mut dispatch = DispatchPlugin::new_from_bytes(bytes, precompiled, plugin_name.clone(), self)?;
        dispatch.initialize()?;
        self.dispatch_plugins.pin().insert(plugin_name, dispatch.into());
        Ok(())
    }

    /// Load a dispatch plugin.
    pub fn load_dispatch_plugin<PathT>(
        &mut self,
        plugin_name: ByteString,
        path: PathT,
        precompiled: bool,
    ) -> Result<(), PluginError>
    where
        StoreT: Clone + Send,
        PathT: AsRef<path::Path>,
    {
        let mut dispatch = DispatchPlugin::new_from_file(path, precompiled, plugin_name.clone(), self)?;
        dispatch.initialize()?;
        self.dispatch_plugins.pin().insert(plugin_name, dispatch.into());
        Ok(())
    }

    /// Get a dispatch plugin.
    pub fn dispatch_plugin(&mut self, plugin_name: &str) -> Result<DispatchPluginRef<StoreT>, PluginError> {
        self.dispatch_plugins.pin().get(plugin_name).cloned().ok_or_else(|| PluginError::NotFound(plugin_name.into()))
    }
}
