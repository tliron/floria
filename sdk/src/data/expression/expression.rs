use super::{
    super::super::{dispatch_bindings::*, errors},
    custom::*,
};

use std::{collections::*, mem::*};

impl Expression {
    /// Gets a reference to an inner [Expression].
    ///
    /// If this is a map, the argument is treated as a key.
    ///
    /// If this is a list, the argument is treated as an index and must be an
    /// [Expression::UnsignedInteger] or an [Expression::Integer].
    pub fn get(&self, key: &Self) -> Option<&Self> {
        match self {
            Self::Map(map_resource) => map_resource.map().inner.get(key),

            Self::List(list_resource) => {
                let index = match key {
                    Self::UnsignedInteger(unsigned_integer) => *unsigned_integer as usize,
                    Self::Integer(integer) => *integer as usize,
                    _ => return None,
                };

                list_resource.list().inner.get(index)
            }

            _ => None,
        }
    }

    /// Gets a reference to an inner [Expression].
    ///
    /// If this is a map, the argument is treated as a key.
    ///
    /// If this is a list, the argument is treated as an index and must be an
    /// [Expression::UnsignedInteger] or an [Expression::Integer].
    pub fn into_get<KeyT>(&self, key: KeyT) -> Option<&Self>
    where
        KeyT: Into<Self>,
    {
        self.get(&key.into())
    }

    /// Traverse this [Expression] by calling [Expression::get] repeatedly.
    ///
    /// Any non-collection or missing key will cause the traversal to stop and return [None].
    ///
    /// Use the [traverse!](crate::traverse) macro instead if you can. It will generally
    /// be more efficient because it doesn't require an allocated array.
    pub fn traverse<'own, IterableT>(&self, keys: IterableT) -> Option<&Self>
    where
        IterableT: IntoIterator<Item = &'own Self>,
    {
        let mut found = self;
        for key in keys {
            found = found.get(key)?;
        }
        Some(found)
    }

    /// Compare type.
    pub fn same_type(&self, other: &Self) -> bool {
        if let Expression::Custom(left) = self
            && let Expression::Custom(right) = other
        {
            left.custom().kind == right.custom().kind
        } else {
            discriminant(self) == discriminant(other)
        }
    }

    /// Assert same type.
    pub fn assert_same_type(&self, other: &Expression, operator: &str) -> Result<(), String> {
        if self.same_type(other) { Ok(()) } else { Err(errors::not_same_type(self, other, operator)) }
    }

    /// Type name.
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Integer(_) => "integer",
            Self::UnsignedInteger(_) => "unsigned integer",
            Self::Float(_) => "float",
            Self::Boolean(_) => "boolean",
            Self::Text(_) => "text",
            Self::Blob(_) => "blob",
            Self::List(_) => "list",
            Self::Map(_) => "map",
            Self::Custom(_) => "custom",
            Self::Call(_) => "call",
        }
    }

    /// Type name article: either "a" or "an".
    pub fn type_name_article(&self) -> &'static str {
        match self {
            Self::Integer(_) | Self::UnsignedInteger(_) => "an",
            _ => "a",
        }
    }

    /// Evaluate an expression.
    ///
    /// Lazy calls are not dispatched here. They must be dispatched manually, e.g. via
    /// [dispatch_if_call](Expression::dispatch_if_call).
    pub fn evaluate(self, call_site: &CallSite) -> Result<Option<Expression>, String> {
        if let Expression::Call(call_resource) = &self
            && matches!(call_resource.call().kind, CallKind::Lazy)
        {
            return Ok(Some(self));
        }

        match self {
            Expression::List(list_resource) => {
                let list = list_resource.list();

                let mut evaluated_list = Vec::with_capacity(list.inner.len());
                for item in &list.inner {
                    let Some(item) = item.clone().evaluate(call_site)? else {
                        return Err(format!("list item evaluated to nothing: |error|{}|", item.type_name()));
                    };

                    evaluated_list.push(item);
                }

                Ok(Some(evaluated_list.into()))
            }

            Expression::Map(map_resource) => {
                let map = map_resource.map();

                let mut evaluated_map = BTreeMap::default();
                for (key, value) in &map.inner {
                    let Some(key) = key.clone().evaluate(call_site)? else {
                        return Err(format!("map entry key evaluated to nothing: |error|{}|", key.type_name()));
                    };

                    let Some(value) = value.clone().evaluate(call_site)? else {
                        return Err(format!("map entry value evaluated to nothing: |error|{}|", value.type_name()));
                    };

                    evaluated_map.insert(key, value);
                }

                Ok(Some(evaluated_map.into()))
            }

            Expression::Custom(custom_resource) => {
                let custom = custom_resource.custom();

                let Some(inner) = custom.inner.clone().evaluate(call_site)? else {
                    return Err(format!("custom evaluated to nothing: |error|{}|", custom.inner.type_name()));
                };

                Ok(Some(Custom::new(custom.kind.clone(), inner).into()))
            }

            Expression::Call(call_resource) => call_resource.call().dispatch(call_site),

            _ => Ok(Some(self)),
        }
    }

    /// Evaluate an expression.
    ///
    /// Returns an error if the expression evaluates to nothing.
    ///
    /// Lazy calls are not dispatched here. They must be dispatched manually, e.g. via
    /// [dispatch_if_call](Expression::dispatch_if_call).
    pub fn must_evaluate(self, call_site: &CallSite) -> Result<Expression, String> {
        let type_name = self.type_name();
        self.evaluate(call_site)?.ok_or_else(|| format!("expression evaluated to nothing: |error|{}|", type_name))
    }

    /// Dispatch if we are a call, otherwise return self.
    pub fn dispatch_if_call(self, call_site: &CallSite) -> Result<Option<Expression>, String> {
        if let Expression::Call(call_resource) = self {
            call_resource.call().dispatch(call_site)
        } else {
            Ok(Some(self))
        }
    }

    /// Dispatch if we are a call, otherwise return self.
    ///
    /// Returns an error if the call returns nothing.
    pub fn must_dispatch_if_call(self, call_site: &CallSite) -> Result<Expression, String> {
        self.dispatch_if_call(call_site)?.ok_or_else(|| "call returned nothing".into())
    }
}

impl Clone for Expression {
    fn clone(&self) -> Self {
        match self {
            Self::Null => Self::Null,
            Self::Integer(integer) => Self::Integer(*integer),
            Self::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(*unsigned_integer),
            Self::Float(float) => Self::Float(*float),
            Self::Boolean(boolean) => Self::Boolean(*boolean),
            Self::Text(text) => Self::Text(text.clone()),
            Self::Blob(blob) => Self::Blob(blob.clone()),
            Self::List(list_resource) => list_resource.list().clone().into(),
            Self::Map(map_resource) => map_resource.map().clone().into(),
            Self::Custom(custom_resource) => custom_resource.custom().clone().into(),
            Self::Call(call_resource) => call_resource.call().clone().into(),
        }
    }
}
