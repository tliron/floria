use super::super::{data, dispatch_bindings, floria_bindings::*};

use std::collections::*;

// Expression

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
                let list: Vec<Self> = list_resource.get().into_iter().map(|item| item.into()).collect();
                list.into()
            }

            Expression::Map(map_resource) => {
                let map: BTreeMap<_, _> =
                    map_resource.get().into_iter().map(|(key, value)| (key.into(), value.into())).collect();
                map.into()
            }

            Expression::Custom(custom_resource) => {
                let (kind, inner) = custom_resource.get();
                data::Custom { kind, inner: inner.into() }.into()
            }

            Expression::Call(call_resource) => {
                let (plugin, function, arguments) = call_resource.get();
                let arguments: Vec<_> = arguments.into_iter().map(|item| item.into()).collect();
                data::Call { plugin, function, arguments }.into()
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
                let list: Vec<_> = list_resource.list().inner.iter().map(|item| item.clone().into()).collect();
                Self::List(ListResource::new(list))
            }

            data::Expression::Map(map_resource) => {
                let key_value_pairs: Vec<_> = map_resource
                    .map()
                    .inner
                    .iter()
                    .map(|(key, value)| (key.clone().into(), value.clone().into()))
                    .collect();
                Self::Map(MapResource::new(key_value_pairs))
            }

            data::Expression::Custom(custom_resource) => {
                let custom = custom_resource.custom();
                Self::Custom(CustomResource::new(&custom.kind, custom.inner.clone().into()))
            }

            data::Expression::Call(call_resource) => {
                let call = call_resource.call();
                let arguments: Vec<_> = call.arguments.iter().map(|item| item.clone().into()).collect();
                Self::Call(CallResource::new(&call.plugin, &call.function, arguments))
            }
        }
    }
}

// Kind

impl From<Kind> for dispatch_bindings::Kind {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Class => Self::Class,
            Kind::VertexTemplate => Self::VertexTemplate,
            Kind::EdgeTemplate => Self::EdgeTemplate,
            Kind::Vertex => Self::Vertex,
            Kind::Edge => Self::Edge,
        }
    }
}

impl From<dispatch_bindings::Kind> for Kind {
    fn from(kind: dispatch_bindings::Kind) -> Self {
        match kind {
            dispatch_bindings::Kind::Class => Self::Class,
            dispatch_bindings::Kind::VertexTemplate => Self::VertexTemplate,
            dispatch_bindings::Kind::EdgeTemplate => Self::EdgeTemplate,
            dispatch_bindings::Kind::Vertex => Self::Vertex,
            dispatch_bindings::Kind::Edge => Self::Edge,
        }
    }
}

// Id

impl From<Id> for dispatch_bindings::Id {
    fn from(id: Id) -> Self {
        Self { kind: id.kind.into(), directory: id.directory, id: id.id }
    }
}

impl From<dispatch_bindings::Id> for Id {
    fn from(id: dispatch_bindings::Id) -> Self {
        Self { kind: id.kind.into(), directory: id.directory, id: id.id }
    }
}

// Site

impl From<CallSite> for dispatch_bindings::CallSite {
    fn from(call_site: CallSite) -> Self {
        Self { id: call_site.id.into(), path: call_site.path }
    }
}

impl From<dispatch_bindings::CallSite> for CallSite {
    fn from(call_site: dispatch_bindings::CallSite) -> Self {
        Self { id: call_site.id.into(), path: call_site.path }
    }
}
