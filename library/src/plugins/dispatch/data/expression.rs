use super::super::{
    super::{
        super::{data::*, store::*},
        bindings::exports::floria::plugins::dispatch as bindings,
        errors::*,
    },
    plugin::*,
};

use {anyhow::Context, std::collections::*};

impl<StoreT> DispatchPlugin<StoreT>
where
    StoreT: Store,
{
    /// Convert an [Expression] to a [bindings::Expression].
    pub fn expression_to_bindings(&mut self, expression: Expression) -> Result<bindings::Expression, PluginError> {
        match expression {
            Expression::Undefined | Expression::Null => Ok(bindings::Expression::Null),
            Expression::Integer(integer) => Ok(bindings::Expression::Integer(integer)),
            Expression::UnsignedInteger(unsigned_integer) => {
                Ok(bindings::Expression::UnsignedInteger(unsigned_integer))
            }
            Expression::Float(float) => Ok(bindings::Expression::Float(float.into())),
            Expression::Boolean(boolean) => Ok(bindings::Expression::Boolean(boolean)),
            Expression::Text(text) => Ok(bindings::Expression::Text(text.into())),
            Expression::Blob(blob) => Ok(bindings::Expression::Blob(blob.into())),

            Expression::List(list) => {
                let mut items = Vec::with_capacity(list.len());
                for item in list.into_iter() {
                    items.push(self.expression_to_bindings(item)?);
                }

                let resource = self
                    .bindings
                    .floria_plugins_dispatch()
                    .list_resource()
                    .call_constructor(&mut self.host, &items)
                    .context("calling list constructor")
                    .map_err(PluginError::CallWasm)?;

                Ok(bindings::Expression::List(resource))
            }

            Expression::Map(map) => {
                let mut key_value_pairs = Vec::with_capacity(map.len());
                for (key, value) in map.into_iter() {
                    key_value_pairs.push((self.expression_to_bindings(key)?, self.expression_to_bindings(value)?));
                }

                let resource = self
                    .bindings
                    .floria_plugins_dispatch()
                    .map_resource()
                    .call_constructor(&mut self.host, &key_value_pairs)
                    .context("calling map constructor")
                    .map_err(PluginError::CallWasm)?;

                Ok(bindings::Expression::Map(resource))
            }

            Expression::Custom(kind, inner) => {
                let inner = self.expression_to_bindings(*inner)?;

                let resource = self
                    .bindings
                    .floria_plugins_dispatch()
                    .custom_resource()
                    .call_constructor(&mut self.host, &kind, &inner)
                    .context("calling custom constructor")
                    .map_err(PluginError::CallWasm)?;

                Ok(bindings::Expression::Custom(resource))
            }

            Expression::Call(call) => {
                let mut arguments = Vec::with_capacity(call.arguments.len());
                for argument in call.arguments.into_iter() {
                    arguments.push(self.expression_to_bindings(argument)?);
                }

                let resource = self
                    .bindings
                    .floria_plugins_dispatch()
                    .call_resource()
                    .call_constructor(
                        &mut self.host,
                        &call.function.plugin_id.to_string(),
                        &call.function.name,
                        &arguments,
                        call.kind.into(),
                    )
                    .context("calling call constructor")
                    .map_err(PluginError::CallWasm)?;

                Ok(bindings::Expression::Call(resource))
            }
        }
    }

    /// Convert a [bindings::Expression] to an [Expression].
    pub fn expression_from_bindings(&mut self, expression: bindings::Expression) -> Result<Expression, PluginError> {
        match expression {
            bindings::Expression::Null => Ok(Expression::Null),
            bindings::Expression::Integer(integer) => Ok(Expression::Integer(integer)),
            bindings::Expression::UnsignedInteger(unsigned_integer) => {
                Ok(Expression::UnsignedInteger(unsigned_integer))
            }
            bindings::Expression::Float(float) => Ok(Expression::Float(float.into())),
            bindings::Expression::Boolean(boolean) => Ok(Expression::Boolean(boolean)),
            bindings::Expression::Text(text) => Ok(Expression::Text(text.into())),
            bindings::Expression::Blob(blob) => Ok(Expression::Blob(blob.into())),

            bindings::Expression::List(resource) => {
                let items = self
                    .bindings
                    .floria_plugins_dispatch()
                    .list_resource()
                    .call_replica(&mut self.host, resource)
                    .context("calling List::replica")
                    .map_err(PluginError::CallWasm)?;

                resource
                    .resource_drop(&mut self.host)
                    .context("dropping List resource")
                    .map_err(PluginError::CallWasm)?;

                let mut list = Vec::with_capacity(items.len());
                for item in items {
                    list.push(self.expression_from_bindings(item)?);
                }

                Ok(Expression::List(list))
            }

            bindings::Expression::Map(resource) => {
                let key_value_pairs = self
                    .bindings
                    .floria_plugins_dispatch()
                    .map_resource()
                    .call_replica(&mut self.host, resource)
                    .context("calling Map::replica")
                    .map_err(PluginError::CallWasm)?;

                resource
                    .resource_drop(&mut self.host)
                    .context("dropping Map resource")
                    .map_err(PluginError::CallWasm)?;

                let mut map = BTreeMap::default();
                for (key, value) in key_value_pairs {
                    map.insert(self.expression_from_bindings(key)?, self.expression_from_bindings(value)?);
                }

                Ok(Expression::Map(map))
            }

            bindings::Expression::Custom(resource) => {
                let (kind, inner) = self
                    .bindings
                    .floria_plugins_dispatch()
                    .custom_resource()
                    .call_replica(&mut self.host, resource)
                    .context("calling Custom::replica")
                    .map_err(PluginError::CallWasm)?;

                resource
                    .resource_drop(&mut self.host)
                    .context("dropping Custom resource")
                    .map_err(PluginError::CallWasm)?;

                let inner = self.expression_from_bindings(inner)?;
                Ok(Expression::Custom(kind.into(), inner.into()))
            }

            bindings::Expression::Call(resource) => {
                let (plugin, function, arguments, kind) = self
                    .bindings
                    .floria_plugins_dispatch()
                    .call_resource()
                    .call_replica(&mut self.host, resource)
                    .context("calling Call::replica")
                    .map_err(PluginError::CallWasm)?;

                resource
                    .resource_drop(&mut self.host)
                    .context("dropping Call resource")
                    .map_err(PluginError::CallWasm)?;

                let mut expressions = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    expressions.push(self.expression_from_bindings(argument)?);
                }

                Ok(Call::new(ID::parse(EntityKind::Plugin, &plugin)?, function.into(), expressions, kind.into())?
                    .into())
            }
        }
    }
}
