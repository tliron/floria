use super::{super::super::store::*, host::*};

use wasmtime_wasi_tls::*;

impl<StoreT> PluginHost<StoreT>
where
    StoreT: Store,
{
    /// Create WASI TLS.
    pub fn new_tls(&mut self) -> WasiTls<'_> {
        WasiTls::new(&self.wasi_tls, &mut self.resources)
    }
}
