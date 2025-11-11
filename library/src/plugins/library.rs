use super::{
    super::{data::*, errors::*, store::*},
    dispatch::*,
    environment::*,
    errors::*,
};

use {
    kutil::std::collections::*,
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
    pub dispatch_plugins: Arc<FastConcurrentHashMap<ID, DispatchPluginRef<StoreT>>>,
}

impl<StoreT> Library<StoreT>
where
    StoreT: Store,
{
    /// Constructor.
    pub fn new(environment: Environment, store: StoreT) -> Self {
        Self { environment, store, dispatch_plugins: Default::default() }
    }

    /// Get a dispatch plugin.
    pub fn dispatch_plugin_ref(&mut self, plugin_id: &ID) -> Result<DispatchPluginRef<StoreT>, FloriaError> {
        self.dispatch_plugins
            .pin()
            .get(plugin_id)
            .cloned()
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()).into())
    }

    /// Add a dispatch plugin.
    pub fn add_dispatch_plugin(
        &mut self,
        plugin_id: ID,
        bytes: &[u8],
        precompiled: bool,
    ) -> Result<DispatchPluginRef<StoreT>, FloriaError>
    where
        StoreT: Clone + Send,
    {
        let mut dispatch = DispatchPlugin::new_from_bytes(bytes, precompiled, plugin_id.clone(), self)?;
        dispatch.initialize()?;
        let dispatch_ref: DispatchPluginRef<_> = dispatch.into();
        self.dispatch_plugins.pin().insert(plugin_id, dispatch_ref.clone());
        Ok(dispatch_ref)
    }

    /// Load a dispatch plugin.
    pub fn load_dispatch_plugin<PathT>(
        &mut self,
        plugin_id: ID,
        path: PathT,
        precompiled: bool,
    ) -> Result<DispatchPluginRef<StoreT>, FloriaError>
    where
        StoreT: Clone + Send,
        PathT: AsRef<path::Path>,
    {
        let mut dispatch = DispatchPlugin::new_from_file(path, precompiled, plugin_id.clone(), self)?;
        dispatch.initialize()?;
        let dispatch_ref: DispatchPluginRef<_> = dispatch.into();
        self.dispatch_plugins.pin().insert(plugin_id, dispatch_ref.clone());
        Ok(dispatch_ref)
    }

    /// Get a dispatch plugin or load it.
    pub fn maybe_load_dispatch_plugin_ref(&mut self, plugin_id: &ID) -> Result<DispatchPluginRef<StoreT>, FloriaError>
    where
        StoreT: Clone + Send,
    {
        let path = match self.dispatch_plugins.pin().get(plugin_id) {
            Some(dispatch) => return Ok(dispatch.clone()),
            None => match self.store.get_plugin(plugin_id)? {
                Some(plugin) => plugin.url.clone(),
                None => return Err(PluginError::NotFound(plugin_id.to_string()).into()),
            },
        };

        let path: &str = &path;
        self.load_dispatch_plugin(plugin_id.clone(), path, false)
    }
}
