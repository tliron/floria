use super::{super::errors::*, errors::*};

use {anyhow::Context, wasmtime::*};

//
// PluginEnvironment
//

/// Wasm environment for plugins.
///
/// Cloning is cheap and clones always refer to the same shared state.
#[derive(Debug, Clone)]
pub struct PluginEnvironment {
    /// Wasmtime engine.
    pub engine: Engine,
}

impl PluginEnvironment {
    /// Constructor.
    pub fn new(debug: bool) -> Result<Self, FloriaError> {
        let mut config = Config::new();

        // https://fitzgen.com/2025/11/19/inliner.html
        config.compiler_inlining(!debug);

        // This *must* be enabled just to load Wasm with debug info, even if we don't use a debugger
        config.debug_info(debug);

        #[cfg(feature = "wasm-debug")]
        config.guest_debug(debug);

        // This *must* be enabled just to load Wasm with backtrace info
        // (The default is to check WASMTIME_BACKTRACE_DETAILS env var)
        config.wasm_backtrace_details(if debug { WasmBacktraceDetails::Enable } else { WasmBacktraceDetails::Disable });

        // This isn't very helpful to us
        // #[cfg(feature = "wasm-debug")]
        // config.coredump_on_trap(debug);

        tracing::debug!("wasmtime configuration:\n{:#?}", config);
        let engine =
            Engine::new(&config).context("initializing wasmtime engine").map_err(PluginError::InstantiateWasm)?;

        Ok(Self { engine })
    }
}
