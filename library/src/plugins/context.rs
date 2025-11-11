use super::{
    super::{data::*, errors::*, store::*},
    dispatch::*,
    environment::*,
    errors::*,
};

use {
    kutil::std::collections::*,
    read_url::*,
    std::{path, sync::*},
};

//
// PluginContext
//

/// Plugin context.
///
/// Cloning is cheap and clones always refer to the same shared state.
#[derive(Clone)]
pub struct PluginContext<StoreT>
where
    StoreT: 'static + Store,
{
    /// Environment.
    pub environment: PluginEnvironment,

    /// Store.
    pub store: StoreT,

    /// URL context.
    pub url_context: UrlContextRef,

    /// Dispatch plugins.
    pub dispatch_plugins: Arc<FastConcurrentHashMap<ID, DispatchPluginRef<StoreT>>>,
}

impl<StoreT> PluginContext<StoreT>
where
    StoreT: Store,
{
    /// Constructor.
    pub fn new(environment: PluginEnvironment, store: StoreT, url_context: UrlContextRef) -> Self {
        Self { environment, store, url_context, dispatch_plugins: Default::default() }
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
    ///
    /// Prefer [load_dispatch_plugin](Self::load_dispatch_plugin) when a path is available, as the
    /// underlying implementation is optimized for this use case (by memory mapping the file).
    ///
    /// Make sure you trust the component when precompiled is true!
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
    ///
    /// Make sure you trust the component when precompiled is true!
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
        let (url, precompiled) = match self.dispatch_plugins.pin().get(plugin_id) {
            Some(dispatch) => return Ok(dispatch.clone()),
            None => match self.store.get_plugin(plugin_id)? {
                Some(plugin) => (plugin.url.clone(), plugin.precompiled),
                None => return Err(PluginError::NotFound(plugin_id.to_string()).into()),
            },
        };

        let url = self.url_context.url(&url).map_err(PluginError::from)?;

        match url.local() {
            Some(path) => self.load_dispatch_plugin(plugin_id.clone(), path, precompiled),

            None => {
                let mut reader = url.open().map_err(PluginError::from)?;
                let mut bytes = Vec::default();
                reader.read_to_end(&mut bytes).map_err(PluginError::from)?;
                self.add_dispatch_plugin(plugin_id.clone(), &bytes, precompiled)
            }
        }
    }
}
