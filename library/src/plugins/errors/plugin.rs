use super::{
    super::{super::errors::*, dispatch::*},
    initialization::*,
};

use {
    depiction::*,
    read_url::*,
    std::{io, sync::*},
    thiserror::*,
    wasmtime::component::*,
};

//
// PluginError
//

/// Plugin error.
#[derive(Debug, Error)]
pub enum PluginError {
    /// Not found.
    #[error("not found: {0}")]
    NotFound(String),

    /// Load Wasm.
    #[error("load Wasm: {0:?}")]
    LoadWasm(wasmtime::Error),

    /// Link Wasm.
    #[error("link Wasm: {0:?}")]
    LinkWasm(wasmtime::Error),

    /// Instantiate Wasm.
    #[cfg(feature = "plugins")]
    #[error("instantiate Wasm: {0:?}")]
    InstantiateWasm(wasmtime::Error),

    /// Call Wasm.
    #[error("call Wasm: {0:?}")]
    CallWasm(wasmtime::Error),

    /// Wasm resource.
    #[error("Wasm resource: {0}")]
    WasmResource(#[from] ResourceTableError),

    /// URL.
    #[error("URL: {0}")]
    URL(#[from] UrlError),

    /// Initialization.
    #[error("initialization: {0}")]
    Initialization(#[from] InitializationError),

    /// Dispatch.
    #[error("dispatch: {0}")]
    Dispatch(#[from] DispatchError),

    /// Concurrency.
    #[error("concurrency: {0}")]
    Concurrency(String),

    /// IO.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// Malformed.
    #[error("malformed: {0}")]
    Malformed(#[from] MalformedError),
}

impl IntoDepictionMarkup for PluginError {
    fn into_depiction_markup(self) -> String {
        match self {
            Self::Dispatch(error) => error.into_depiction_markup(),
            _ => escape_depiction_markup(self),
        }
    }
}

impl Depict for PluginError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Dispatch(dispatch) => dispatch.depict(writer, context),

            _ => {
                context.separate(writer)?;
                context.theme.write_error(writer, self)
            }
        }
    }
}

impl<GuardT> From<PoisonError<GuardT>> for PluginError {
    fn from(error: PoisonError<GuardT>) -> Self {
        Self::Concurrency(error.to_string())
    }
}
