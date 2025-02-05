use super::{
    super::{
        super::{data::*, store::*},
        bindings::floria::plugins::floria as bindings,
    },
    host::*,
};

use kutil::{cli::depict::escape_depiction_markup, std::error::*};

impl<StoreT> bindings::Host for PluginHost<StoreT>
where
    StoreT: Clone + Send + Store,
{
    fn log(&mut self, source: String, message: String) -> wasmtime::Result<()> {
        tracing::info!("[{}] {}: {}", self.name, source, message);
        Ok(())
    }

    fn evaluate_expression(
        &mut self,
        expression: bindings::Expression,
        call_site: bindings::CallSite,
    ) -> wasmtime::Result<Result<Option<bindings::Expression>, String>> {
        // TODO: also need to make sure we're not calling into same plugin
        let expression = self.expression_from_bindings(expression)?;

        Ok(Ok(match expression.evaluate(&call_site.into(), &mut self.library, &mut FailFastErrorRecipient)? {
            Some(value) => Some(self.expression_to_bindings(value)?),
            None => None,
        }))
    }

    fn get_entity(&mut self, id: bindings::Id) -> wasmtime::Result<Result<bindings::Entity, String>> {
        let id: ID = id.into();

        let entity: Option<bindings::Entity> = match id.kind {
            EntityKind::Vertex => match self.library.store.get_vertex(&id)? {
                Some(vertex) => Some(self.vertex_to_bindings(vertex)?.into()),
                None => None,
            },

            EntityKind::Edge => match self.library.store.get_edge(&id)? {
                Some(edge) => Some(self.edge_to_bindings(edge)?.into()),
                None => None,
            },

            _ => todo!(),
        };

        Ok(match entity {
            Some(entity) => Ok(entity),
            None => Err(format!("entity not found: |error|{}|", escape_depiction_markup(id))),
        })
    }
}
