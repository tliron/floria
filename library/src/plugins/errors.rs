use {problemo::*, wasmtime::component::*};

tag_error!(PluginError, "Floria plugin");
tag_error!(WasmError, "Wasm");

static_gloss_error!(CallError, "call");
static_gloss_error!(LinkError, "link");
static_gloss_error!(LoadError, "load");
static_gloss_error!(ResourceError, "resource");

//
// WasmResult
//

/// Wasm result.
pub trait WasmResult<OkT> {
    /// Via [CallError] and [WasmError] and [PluginError].
    fn into_wasm_call_problem(self, message: &'static str) -> Result<OkT, Problem>;

    /// Via [ResourceError] and [WasmError] and [PluginError].
    fn into_wasm_resource_problem(self, message: &'static str) -> Result<OkT, Problem>;

    /// Via [LinkError] and [WasmError] and [PluginError].
    fn into_wasm_link_problem(self, message: &'static str) -> Result<OkT, Problem>;

    /// Via [LoadError] and [WasmError] and [PluginError].
    fn into_wasm_load_problem(self, message: &'static str) -> Result<OkT, Problem>;
}

impl<ResultT, OkT> WasmResult<OkT> for ResultT
where
    ResultT: AnyhowIntoProblemResult<OkT>,
{
    #[track_caller]
    fn into_wasm_call_problem(self, message: &'static str) -> Result<OkT, Problem> {
        self.into_problem().via(CallError::new(message)).via(WasmError).via(PluginError)
    }

    #[track_caller]
    fn into_wasm_resource_problem(self, message: &'static str) -> Result<OkT, Problem> {
        self.into_problem().via(ResourceError::new(message)).via(WasmError).via(PluginError)
    }

    #[track_caller]
    fn into_wasm_link_problem(self, message: &'static str) -> Result<OkT, Problem> {
        self.into_problem().via(LinkError::new(message)).via(WasmError).via(PluginError)
    }

    #[track_caller]
    fn into_wasm_load_problem(self, message: &'static str) -> Result<OkT, Problem> {
        self.into_problem().via(LoadError::new(message)).via(WasmError).via(PluginError)
    }
}

//
// ResourceTableResult
//

/// Wasm resource table result.
pub trait ResourceTableResult<OkT> {
    /// Via [ResourceError] and [WasmError] and [PluginError].
    fn into_wasm_resource_problem(self, message: &'static str) -> Result<OkT, Problem>;
}

impl<OkT> ResourceTableResult<OkT> for Result<OkT, ResourceTableError> {
    #[track_caller]
    fn into_wasm_resource_problem(self, message: &'static str) -> Result<OkT, Problem> {
        match self.into_problem() {
            Ok(ok) => Ok(ok),
            Err(problem) => Err(problem.via(ResourceError::new(message)).via(WasmError).via(PluginError)),
        }
    }
}
