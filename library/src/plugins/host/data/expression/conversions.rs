use super::super::super::{
    super::{
        super::{data::*, store::*},
        bindings::floria::plugins::floria as bindings,
        errors::*,
    },
    host::*,
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
            Expression::Integer(integer) => Ok(integer.into()),
            Expression::UnsignedInteger(unsigned_integer) => Ok(unsigned_integer.into()),
            Expression::Float(float) => Ok(float.into()),
            Expression::Boolean(boolean) => Ok(boolean.into()),
            Expression::Text(text) => Ok(text.into()),
            Expression::Blob(blob) => Ok(blob.into()),

            Expression::List(list) => {
                let list = self.expressions_to_bindings(list)?;
                let list = bindings::HostListResource::new(self, list).map_err(PluginError::CallWasm)?;
                Ok(list.into())
            }

            Expression::Map(map) => {
                let map = self.expression_map_to_bindings(map)?;
                let map = bindings::HostMapResource::new(self, map).map_err(PluginError::CallWasm)?;
                Ok(map.into())
            }

            Expression::Custom(kind, inner) => {
                let inner = self.expression_to_bindings(*inner)?;
                let custom =
                    bindings::HostCustomResource::new(self, kind.into(), inner).map_err(PluginError::CallWasm)?;
                Ok(custom.into())
            }

            Expression::Call(call) => {
                let arguments = self.expressions_to_bindings(call.arguments)?;
                let call = bindings::HostCallResource::new(
                    self,
                    call.function.plugin_id.to_string().into(),
                    call.function.name.into(),
                    arguments,
                    call.kind.into(),
                )
                .map_err(PluginError::CallWasm)?;
                Ok(call.into())
            }
        }
    }

    /// Convert an expression from bindings.
    pub fn expression_from_bindings(&mut self, expression: bindings::Expression) -> Result<Expression, PluginError> {
        match expression {
            bindings::Expression::Null => Ok(Expression::Null),
            bindings::Expression::Integer(integer) => Ok(integer.into()),
            bindings::Expression::UnsignedInteger(unsigned_integer) => Ok(unsigned_integer.into()),
            bindings::Expression::Float(float) => Ok(float.into()),
            bindings::Expression::Boolean(boolean) => Ok(boolean.into()),
            bindings::Expression::Text(text) => Ok(text.into()),
            bindings::Expression::Blob(blob) => Ok(blob.into()),

            bindings::Expression::List(resource) => {
                let list = self.resources.delete(resource)?.inner;
                let list = self.expressions_from_bindings(list)?;
                Ok(list.into())
            }

            bindings::Expression::Map(resource) => {
                let map = self.resources.delete(resource)?.inner;
                let map = self.expression_map_from_bindings(map)?;
                Ok(map.into())
            }

            bindings::Expression::Custom(resource) => {
                let custom = self.resources.delete(resource)?;
                let inner = self.expression_from_bindings(custom.inner)?;
                Ok(Expression::Custom(custom.kind.into(), inner.into()))
            }

            bindings::Expression::Call(resource) => {
                let call = self.resources.delete(resource)?;
                let plugin = ID::parse(EntityKind::Plugin, &call.plugin)?;
                let function = call.function.into();
                let arguments = self.expressions_from_bindings(call.arguments)?;
                let kind = call.kind.into();
                Ok(Call::new(plugin, function, arguments, kind)?.into())
            }
        }
    }

    /// Convert expressions to bindings.
    pub fn expressions_to_bindings(
        &mut self,
        expressions: Vec<Expression>,
    ) -> Result<Vec<bindings::Expression>, PluginError> {
        let mut expression_bindings = Vec::with_capacity(expressions.len());
        for item in expressions {
            expression_bindings.push(self.expression_to_bindings(item)?);
        }
        Ok(expression_bindings)
    }

    /// Convert expressions from bindings.
    pub fn expressions_from_bindings(
        &mut self,
        expressions: Vec<bindings::Expression>,
    ) -> Result<Vec<Expression>, PluginError> {
        let mut expressions_ = Vec::with_capacity(expressions.len());
        for item in expressions {
            expressions_.push(self.expression_from_bindings(item)?);
        }
        Ok(expressions_)
    }

    /// Convert expression map to bindings.
    pub fn expression_map_to_bindings(
        &mut self,
        expressions: BTreeMap<Expression, Expression>,
    ) -> Result<Vec<(bindings::Expression, bindings::Expression)>, PluginError> {
        let mut expression_bindings = Vec::with_capacity(expressions.len());
        for (key, value) in expressions {
            expression_bindings.push((self.expression_to_bindings(key)?, self.expression_to_bindings(value)?));
        }
        Ok(expression_bindings)
    }

    /// Convert expression map from bindings.
    pub fn expression_map_from_bindings(
        &mut self,
        expressions: Vec<(bindings::Expression, bindings::Expression)>,
    ) -> Result<BTreeMap<Expression, Expression>, PluginError> {
        let mut expressions_ = BTreeMap::default();
        for (key, value) in expressions {
            expressions_.insert(self.expression_from_bindings(key)?, self.expression_from_bindings(value)?);
        }
        Ok(expressions_)
    }

    /// Convert an expression option to bindings.
    pub fn expression_option_to_bindings(
        &mut self,
        expression: Option<Expression>,
    ) -> Result<Option<bindings::Expression>, PluginError> {
        Ok(match expression {
            Some(expression) => Some(self.expression_to_bindings(expression)?),
            None => None,
        })
    }

    /// Convert an expression option from bindings.
    pub fn expression_option_from_bindings(
        &mut self,
        expression: Option<bindings::Expression>,
    ) -> Result<Option<Expression>, PluginError> {
        Ok(match expression {
            Some(expression) => Some(self.expression_from_bindings(expression)?),
            None => None,
        })
    }
}
