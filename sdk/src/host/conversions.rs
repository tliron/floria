use super::super::{data, dispatch_bindings, floria_bindings::*};

use std::collections::*;

// Id

impl From<Id> for dispatch_bindings::Id {
    fn from(id: Id) -> Self {
        Self { kind: id.kind.into(), directory: id.directory, name: id.name }
    }
}

impl From<dispatch_bindings::Id> for Id {
    fn from(id: dispatch_bindings::Id) -> Self {
        Self { kind: id.kind.into(), directory: id.directory, name: id.name }
    }
}

// EntityKind

impl From<EntityKind> for dispatch_bindings::EntityKind {
    fn from(kind: EntityKind) -> Self {
        match kind {
            EntityKind::Plugin => Self::Plugin,
            EntityKind::Class => Self::Class,
            EntityKind::VertexTemplate => Self::VertexTemplate,
            EntityKind::EdgeTemplate => Self::EdgeTemplate,
            EntityKind::Vertex => Self::Vertex,
            EntityKind::Edge => Self::Edge,
        }
    }
}

impl From<dispatch_bindings::EntityKind> for EntityKind {
    fn from(kind: dispatch_bindings::EntityKind) -> Self {
        match kind {
            dispatch_bindings::EntityKind::Plugin => Self::Plugin,
            dispatch_bindings::EntityKind::Class => Self::Class,
            dispatch_bindings::EntityKind::VertexTemplate => Self::VertexTemplate,
            dispatch_bindings::EntityKind::EdgeTemplate => Self::EdgeTemplate,
            dispatch_bindings::EntityKind::Vertex => Self::Vertex,
            dispatch_bindings::EntityKind::Edge => Self::Edge,
        }
    }
}

// Expression

impl From<&Expression> for data::Expression {
    fn from(expression: &Expression) -> Self {
        match expression {
            Expression::Null => Self::Null,
            Expression::Integer(integer) => Self::Integer(*integer),
            Expression::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(*unsigned_integer),
            Expression::Float(float) => Self::Float(*float),
            Expression::Boolean(boolean) => Self::Boolean(*boolean),
            Expression::Text(text) => Self::Text(text.clone()),
            Expression::Blob(blob) => Self::Blob(blob.clone()),

            Expression::List(list_resource) => {
                let list: Vec<Self> = list_resource.take().into_iter().map(|item| item.into()).collect();
                list.into()
            }

            Expression::Map(map_resource) => {
                let map: BTreeMap<_, _> =
                    map_resource.take().into_iter().map(|(key, value)| (key.into(), value.into())).collect();
                map.into()
            }

            Expression::Custom(custom_resource) => {
                let (kind, inner) = custom_resource.take();
                data::Custom { kind, inner: inner.into() }.into()
            }

            Expression::Call(call_resource) => {
                let (plugin, function, arguments, kind) = call_resource.take();
                let arguments: Vec<_> = arguments.into_iter().map(|item| item.into()).collect();
                data::Call { plugin, function, arguments, kind: kind.into() }.into()
            }
        }
    }
}

impl From<Expression> for data::Expression {
    fn from(expression: Expression) -> Self {
        match expression {
            Expression::Null => Self::Null,
            Expression::Integer(integer) => Self::Integer(integer),
            Expression::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(unsigned_integer),
            Expression::Float(float) => Self::Float(float),
            Expression::Boolean(boolean) => Self::Boolean(boolean),
            Expression::Text(text) => Self::Text(text),
            Expression::Blob(blob) => Self::Blob(blob),

            Expression::List(list_resource) => {
                let list: Vec<Self> = list_resource.take().into_iter().map(|item| item.into()).collect();
                list.into()
            }

            Expression::Map(map_resource) => {
                let map: BTreeMap<_, _> =
                    map_resource.take().into_iter().map(|(key, value)| (key.into(), value.into())).collect();
                map.into()
            }

            Expression::Custom(custom_resource) => {
                let (kind, inner) = custom_resource.take();
                data::Custom { kind, inner: inner.into() }.into()
            }

            Expression::Call(call_resource) => {
                let (plugin, function, arguments, kind) = call_resource.take();
                let arguments: Vec<_> = arguments.into_iter().map(|item| item.into()).collect();
                data::Call { plugin, function, arguments, kind: kind.into() }.into()
            }
        }
    }
}

impl From<data::Expression> for Expression {
    fn from(expression: data::Expression) -> Self {
        match expression {
            data::Expression::Null => Self::Null,
            data::Expression::Integer(integer) => Self::Integer(integer),
            data::Expression::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(unsigned_integer),
            data::Expression::Float(float) => Self::Float(float),
            data::Expression::Boolean(boolean) => Self::Boolean(boolean),
            data::Expression::Text(text) => Self::Text(text),
            data::Expression::Blob(blob) => Self::Blob(blob),

            data::Expression::List(list_resource) => {
                let list: Vec<_> = list_resource.into_list().inner.into_iter().map(|item| item.into()).collect();
                Self::List(ListResource::new(list))
            }

            data::Expression::Map(map_resource) => {
                let key_value_pairs: Vec<_> =
                    map_resource.into_map().inner.into_iter().map(|(key, value)| (key.into(), value.into())).collect();
                Self::Map(MapResource::new(key_value_pairs))
            }

            data::Expression::Custom(custom_resource) => {
                let custom = custom_resource.into_custom();
                Self::Custom(CustomResource::new(&custom.kind, custom.inner.into()))
            }

            data::Expression::Call(call_resource) => {
                let call = call_resource.into_call();
                let arguments: Vec<_> = call.arguments.into_iter().map(|item| item.into()).collect();
                Self::Call(CallResource::new(&call.plugin, &call.function, arguments, call.kind.into()))
            }
        }
    }
}

// CallKind

impl From<CallKind> for dispatch_bindings::CallKind {
    fn from(kind: CallKind) -> Self {
        match kind {
            CallKind::Normal => Self::Normal,
            CallKind::Eager => Self::Eager,
            CallKind::Lazy => Self::Lazy,
        }
    }
}

impl From<dispatch_bindings::CallKind> for CallKind {
    fn from(kind: dispatch_bindings::CallKind) -> Self {
        match kind {
            dispatch_bindings::CallKind::Normal => Self::Normal,
            dispatch_bindings::CallKind::Eager => Self::Eager,
            dispatch_bindings::CallKind::Lazy => Self::Lazy,
        }
    }
}

// CallSite

impl From<CallSite> for dispatch_bindings::CallSite {
    fn from(call_site: CallSite) -> Self {
        Self::new(call_site.id.into(), call_site.property)
    }
}

impl From<dispatch_bindings::CallSite> for CallSite {
    fn from(call_site: dispatch_bindings::CallSite) -> Self {
        Self { id: call_site.id.into(), property: call_site.property }
    }
}
