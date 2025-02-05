use super::super::{
    super::{data::*, store::*},
    bindings::floria::plugins::floria as host,
    library::*,
};

use {compris::annotate::*, kutil::std::immutable::*, wasmtime_wasi::*};

//
// PluginHost
//

/// Floria plugin host.
pub struct PluginHost<StoreT>
where
    StoreT: 'static + Store,
{
    /// Name.
    pub name: ByteString,

    /// Library.
    pub library: Library<StoreT>,

    wasi: WasiCtx,
    pub(crate) resources: ResourceTable,
}

impl<StoreT> PluginHost<StoreT>
where
    StoreT: 'static + Store,
{
    /// Constructor.
    pub fn new(name: ByteString, library: Library<StoreT>) -> Self {
        Self {
            name,
            library,
            wasi: WasiCtxBuilder::new().inherit_stdout().inherit_stderr().build(),
            resources: ResourceTable::default(),
        }
    }
}

impl<StoreT> WasiView for PluginHost<StoreT>
where
    StoreT: 'static + Send + Store,
{
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView { ctx: &mut self.wasi, table: &mut self.resources }
    }
}

impl<StoreT> host::Host for PluginHost<StoreT>
where
    StoreT: 'static + Store + Clone + Send,
{
    fn log(&mut self, source: String, message: String) -> Result<(), wasmtime::Error> {
        tracing::info!("[{}] {}: {}", self.name, source, message);
        Ok(())
    }

    fn evaluate_expression(
        &mut self,
        _expression: host::Any,
        site: host::Site,
    ) -> Result<Result<host::Any, String>, wasmtime::Error> {
        // TODO: also need to make sure we're not calling into same plugin
        let expression = Expression::default();
        let value = expression.evaluate::<_, WithAnnotations>(&site.into(), &mut self.library).unwrap();
        let value = self.to_host_any(value)?;
        Ok(Ok(value))
    }

    fn get_entity(&mut self, id: host::Id) -> Result<Result<host::Any, String>, wasmtime::Error> {
        Ok(match self.library.store.get_entity_as_variant::<WithoutAnnotations>(&id.into())? {
            Some(value) => Ok(self.to_host_any(value)?),
            None => todo!(),
        })
    }
}
