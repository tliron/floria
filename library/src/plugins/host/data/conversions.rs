use super::super::super::{super::data::*, bindings::floria::plugins::floria as bindings, dispatch::*};

// Kind

impl From<bindings::Kind> for Kind {
    fn from(kind: bindings::Kind) -> Self {
        match kind {
            bindings::Kind::Class => Self::Class,
            bindings::Kind::VertexTemplate => Self::VertexTemplate,
            bindings::Kind::EdgeTemplate => Self::EdgeTemplate,
            bindings::Kind::Vertex => Self::Vertex,
            bindings::Kind::Edge => Self::Edge,
        }
    }
}

impl From<Kind> for bindings::Kind {
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

// ID

impl From<bindings::Id> for ID {
    fn from(id: bindings::Id) -> Self {
        let directory = id.directory.into_iter().map(|segment| segment.into()).collect();
        Self::new_for(id.kind.into(), directory, id.id.into())
    }
}

impl From<ID> for bindings::Id {
    fn from(id: ID) -> Self {
        let directory = id.directory.into_iter().map(|segment| segment.into()).collect();
        Self { kind: id.kind.into(), directory, id: id.id.into() }
    }
}

// CallSite

impl From<bindings::CallSite> for CallSite {
    fn from(call_site: bindings::CallSite) -> Self {
        Self::new(call_site.id.into(), call_site.path)
    }
}

impl From<CallSite> for bindings::CallSite {
    fn from(call_site: CallSite) -> Self {
        let id: ID = call_site.id.into();
        Self { id: id.into(), path: call_site.path }
    }
}
