use super::super::super::super::{super::store::*, bindings::floria::plugins::floria as bindings, errors::*, host::*};

use {problemo::*, wasmtime::component::*};

impl<StoreT> PluginHost<StoreT>
where
    StoreT: Store,
{
    /// Deep clone an expression.
    ///
    /// Internal resources will be cloned while simple values will be moved.
    ///
    /// Note that this function accepts a value rather than a reference. Because you likely have a
    /// reference, you can call [shallow_clone](ShallowClone::shallow_clone) on it first.
    pub fn deep_clone_expression(&mut self, expression: bindings::Expression) -> Result<bindings::Expression, Problem> {
        Ok(match expression {
            bindings::Expression::List(resource) => {
                let list = self.resources.get(&resource)?.inner.shallow_clone();
                let list = self.deep_clone_expressions(list)?;
                let list = bindings::HostListResource::new(self, list).into_wasm_resource_problem("new list")?;
                list.into()
            }

            bindings::Expression::Map(resource) => {
                let key_value_pairs = self.resources.get(&resource)?.inner.shallow_clone();
                let key_value_pairs = self.deep_clone_expression_pairs(key_value_pairs)?;
                let map =
                    bindings::HostMapResource::new(self, key_value_pairs).into_wasm_resource_problem("new map")?;
                map.into()
            }

            bindings::Expression::Custom(resource) => {
                let custom = self.resources.get(&resource)?;
                let kind = custom.kind.clone();
                let inner = self.deep_clone_expression(custom.inner.shallow_clone())?;
                let custom =
                    bindings::HostCustomResource::new(self, kind, inner).into_wasm_resource_problem("new custom")?;
                custom.into()
            }

            bindings::Expression::Call(resource) => {
                let call = self.resources.get(&resource)?;
                let plugin = call.plugin.clone();
                let function = call.function.clone();
                let kind = call.kind.into();
                let arguments = self.deep_clone_expressions(call.arguments.shallow_clone())?;
                let call = bindings::HostCallResource::new(self, plugin, function, arguments, kind)
                    .into_wasm_resource_problem("new call")?;
                call.into()
            }

            _ => expression,
        })
    }

    /// Deep clone expressions.
    pub fn deep_clone_expressions(
        &mut self,
        expressions: Vec<bindings::Expression>,
    ) -> Result<Vec<bindings::Expression>, Problem> {
        let mut cloned = Vec::with_capacity(expressions.len());
        for expression in expressions {
            cloned.push(self.deep_clone_expression(expression)?);
        }
        Ok(cloned)
    }

    /// Deep clone expression pairs.
    pub fn deep_clone_expression_pairs(
        &mut self,
        expressions: Vec<(bindings::Expression, bindings::Expression)>,
    ) -> Result<Vec<(bindings::Expression, bindings::Expression)>, Problem> {
        let mut cloned = Vec::with_capacity(expressions.len());
        for (key, value) in expressions {
            cloned.push((self.deep_clone_expression(key)?, self.deep_clone_expression(value)?));
        }
        Ok(cloned)
    }
}

//
// ShallowClone
//

/// Shallow clone.
pub trait ShallowClone {
    /// Shallow clone.
    ///
    /// Does *not* clone internal resources, meaning that they *can* be dropped, which will result
    /// in errors when trying to access the clone.
    ///
    /// Thus you *must* call [deep_clone_expression](PluginHost::deep_clone_expression) on a
    /// shallow clone.
    fn shallow_clone(&self) -> Self;
}

impl ShallowClone for bindings::Expression {
    fn shallow_clone(&self) -> Self {
        match self {
            Self::Null => Self::Null,
            Self::Integer(integer) => Self::Integer(*integer),
            Self::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(*unsigned_integer),
            Self::Float(float) => Self::Float(*float),
            Self::Boolean(boolean) => Self::Boolean(*boolean),
            Self::Text(text) => Self::Text(text.clone()),
            Self::Blob(blob) => Self::Blob(blob.clone()),
            Self::List(resource) => Self::List(if resource.owned() {
                Resource::new_own(resource.rep())
            } else {
                Resource::new_borrow(resource.rep())
            }),
            Self::Map(resource) => Self::Map(if resource.owned() {
                Resource::new_own(resource.rep())
            } else {
                Resource::new_borrow(resource.rep())
            }),
            Self::Custom(resource) => Self::Custom(if resource.owned() {
                Resource::new_own(resource.rep())
            } else {
                Resource::new_borrow(resource.rep())
            }),
            Self::Call(resource) => Self::Call(if resource.owned() {
                Resource::new_own(resource.rep())
            } else {
                Resource::new_borrow(resource.rep())
            }),
        }
    }
}

impl ShallowClone for Vec<bindings::Expression> {
    fn shallow_clone(&self) -> Self {
        self.into_iter().map(|expression| expression.shallow_clone()).collect()
    }
}

impl ShallowClone for Vec<(bindings::Expression, bindings::Expression)> {
    fn shallow_clone(&self) -> Self {
        self.into_iter().map(|(key, value)| (key.shallow_clone(), value.shallow_clone())).collect()
    }
}
