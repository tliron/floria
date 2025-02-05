use super::super::super::dispatch_bindings::*;

use std::mem::*;

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
        if let Expression::Custom(custom1) = self
            && let Expression::Custom(custom2) = other
        {
            custom1.custom().kind == custom2.custom().kind
        } else {
            discriminant(self) == discriminant(other)
        }
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
