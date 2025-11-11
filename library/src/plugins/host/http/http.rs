use super::{
    super::{super::super::store::*, host::*},
    request::*,
};

use {
    wasmtime_wasi::{runtime::*, *},
    wasmtime_wasi_http::{body::*, types::*, *},
};

impl<StoreT> WasiHttpView for PluginHost<StoreT>
where
    StoreT: Store,
{
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.wasi_http
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resources
    }

    fn send_request(
        &mut self,
        request: hyper::Request<HyperOutgoingBody>,
        config: OutgoingRequestConfig,
    ) -> HttpResult<HostFutureIncomingResponse> {
        let url_context = self.context.url_context.clone();
        let handle = spawn(async move { Ok(send_request(request, config, url_context).await) });
        Ok(HostFutureIncomingResponse::pending(handle))
    }
}
