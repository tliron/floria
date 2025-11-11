use super::{
    super::{
        super::{data::*, errors::*, store::*},
        bindings::floria::plugins::floria as bindings,
    },
    host::*,
};

use {depiction::*, kutil::std::error::*};

impl<StoreT> bindings::Host for PluginHost<StoreT>
where
    StoreT: Clone + Send + Store,
{
    fn log(&mut self, source: String, message: String) -> wasmtime::Result<()> {
        tracing::info!("[{}] {}: {}", self.id, source, message);
        Ok(())
    }

    fn evaluate_expression(
        &mut self,
        expression: bindings::Expression,
        call_site: bindings::CallSite,
    ) -> wasmtime::Result<Result<Option<bindings::Expression>, String>> {
        Ok(self._evaluate_expression(expression, call_site).map_err(|error| error.into_depiction_markup()))
    }

    fn get_entity(&mut self, id: bindings::Id) -> wasmtime::Result<Result<bindings::Entity, String>> {
        Ok(self._get_entity(id).map_err(|error| error.into_depiction_markup()))
    }

    fn add_entity(&mut self, entity: bindings::Entity) -> wasmtime::Result<Result<(), String>> {
        Ok(self._add_entity(entity).map_err(|error| error.into_depiction_markup()))
    }
}

impl<StoreT> PluginHost<StoreT>
where
    StoreT: Clone + Send + Store,
{
    fn _evaluate_expression(
        &mut self,
        expression: bindings::Expression,
        call_site: bindings::CallSite,
    ) -> Result<Option<bindings::Expression>, FloriaError> {
        // TODO: also need to make sure we're not calling into same plugin

        let mut expression = self.expression_from_bindings(expression)?;
        if let Expression::Call(call) = &mut expression {
            call.kind = CallKind::Eager;
        }

        Ok(match expression.evaluate(&call_site.try_into()?, &mut self.context, &mut FailFastErrorReceiver)? {
            Some(expression) => Some(self.expression_to_bindings(expression)?),
            None => None,
        })
    }

    fn _get_entity(&mut self, id: bindings::Id) -> Result<bindings::Entity, FloriaError> {
        let id: ID = id.try_into()?;

        let entity: Option<bindings::Entity> = match id.kind {
            EntityKind::Vertex => match self.context.store.get_vertex(&id)? {
                Some(vertex) => Some(self.vertex_to_bindings(vertex)?.into()),
                None => None,
            },

            EntityKind::Edge => match self.context.store.get_edge(&id)? {
                Some(edge) => Some(self.edge_to_bindings(edge)?.into()),
                None => None,
            },

            _ => todo!(),
        };

        match entity {
            Some(entity) => Ok(entity),
            None => {
                // TODO: new error type
                Err(FloriaError::Instantiation(format!("entity not found: |error|{}|", escape_depiction_markup(id))))
            }
        }
    }

    fn _add_entity(&mut self, entity: bindings::Entity) -> Result<(), FloriaError> {
        match entity {
            bindings::Entity::Vertex(vertex) => {
                let vertex = self.vertex_from_bindings(vertex)?;
                Ok(self.context.store.add_vertex(vertex)?)
            }

            bindings::Entity::Edge(_edge) => todo!(),
        }
    }
}
