use super::super::{
    super::{
        super::{
            super::{data::*, errors::*, store::*},
            bindings::floria::plugins::floria as bindings,
            errors::*,
        },
        host::*,
    },
    call::Call as BindingsCall,
    custom::Custom as BindingsCustom,
    list::List as BindingsList,
    map::Map as BindingsMap,
};

use std::collections::*;

impl<StoreT> PluginHost<StoreT>
where
    StoreT: Store,
{
    /// Convert an expression to bindings.
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
                for item in list {
                    items.push(self.expression_to_bindings(item)?);
                }

                let resource = self.resources.push(BindingsList::new(items))?;
                Ok(bindings::Expression::List(resource))
            }

            Expression::Map(map) => {
                let mut key_value_pairs = Vec::with_capacity(map.len());
                for (key, value) in map {
                    key_value_pairs.push((self.expression_to_bindings(key)?, self.expression_to_bindings(value)?));
                }

                let resource = self.resources.push(BindingsMap::new(key_value_pairs))?;
                Ok(bindings::Expression::Map(resource))
            }

            Expression::Custom(kind, inner) => {
                let inner = self.expression_to_bindings(*inner)?;
                let resource = self.resources.push(BindingsCustom::new(kind.into(), inner))?;
                Ok(bindings::Expression::Custom(resource))
            }

            Expression::Call(call) => {
                let mut arguments = Vec::with_capacity(call.arguments.len());
                for argument in call.arguments.into_iter() {
                    arguments.push(self.expression_to_bindings(argument)?);
                }

                let resource = self.resources.push(BindingsCall::new(
                    call.function.plugin_id.to_string().into(),
                    call.function.name.into(),
                    arguments,
                    call.kind.into(),
                ))?;

                Ok(bindings::Expression::Call(resource))
            }
        }
    }

    /// Convert an expression from bindings.
    pub fn expression_from_bindings(&mut self, expression: bindings::Expression) -> Result<Expression, FloriaError> {
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
                let list = self.resources.get(&resource).map_err(PluginError::WasmResource)?.list.clone();

                let mut items = Vec::with_capacity(list.len());
                for item in list {
                    items.push(self.expression_from_bindings(item)?);
                }

                Ok(Expression::List(items))
            }

            bindings::Expression::Map(resource) => {
                let key_value_pairs =
                    self.resources.get(&resource).map_err(PluginError::WasmResource)?.key_value_pairs.clone();

                let mut map = BTreeMap::default();
                for (key, value) in key_value_pairs {
                    map.insert(self.expression_from_bindings(key)?, self.expression_from_bindings(value)?);
                }

                Ok(Expression::Map(map))
            }

            bindings::Expression::Custom(resource) => {
                let custom = self.resources.get(&resource).map_err(PluginError::WasmResource)?.clone();
                let inner = self.expression_from_bindings(custom.inner)?;
                Ok(Expression::Custom(custom.kind.into(), inner.into()))
            }

            bindings::Expression::Call(resource) => {
                let call = self.resources.get(&resource).map_err(PluginError::WasmResource)?;
                let plugin = ID::parse(EntityKind::Plugin, &call.plugin)?;
                let function = call.function.clone().into();
                let arguments = call.arguments.clone();
                let kind = call.kind.into();

                let mut expression_arguments = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    expression_arguments.push(self.expression_from_bindings(argument)?);
                }

                Ok(Call::new(plugin, function, expression_arguments, kind)?.into())
            }
        }
    }
}
