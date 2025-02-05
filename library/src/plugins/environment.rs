use super::errors::*;

//
// Environment
//

/// Wasm environment for plugins.
///
/// Cloning is cheap and clones always refer to the same shared state.
#[derive(Debug, Clone)]
pub struct Environment {
    /// Wasmtime engine.
    pub engine: wasmtime::Engine,
}

impl Environment {
    /// Constructor.
    pub fn new(debug: bool) -> Result<Self, PluginError> {
        let mut config = wasmtime::Config::new();
        config.debug_info(debug).memory_init_cow(true);

        tracing::debug!("wasmtime configuration:\n{:#?}", config);
        let engine = wasmtime::Engine::new(&config).map_err(PluginError::InstantiateWasm)?;

        Ok(Self { engine })
    }
}
