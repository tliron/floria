use super::super::super::{super::data::*, bindings::exports::floria::plugins::dispatch as bindings};

// Kind

impl From<bindings::EntityKind> for EntityKind {
    fn from(kind: bindings::EntityKind) -> Self {
        match kind {
            bindings::EntityKind::Class => Self::Class,
            bindings::EntityKind::VertexTemplate => Self::VertexTemplate,
            bindings::EntityKind::EdgeTemplate => Self::EdgeTemplate,
            bindings::EntityKind::Vertex => Self::Vertex,
            bindings::EntityKind::Edge => Self::Edge,
        }
    }
}

impl From<EntityKind> for bindings::EntityKind {
    fn from(kind: EntityKind) -> Self {
        match kind {
            EntityKind::Class => Self::Class,
            EntityKind::VertexTemplate => Self::VertexTemplate,
            EntityKind::EdgeTemplate => Self::EdgeTemplate,
            EntityKind::Vertex => Self::Vertex,
            EntityKind::Edge => Self::Edge,
        }
    }
}

// CallKind

impl From<bindings::CallKind> for CallKind {
    fn from(kind: bindings::CallKind) -> Self {
        match kind {
            bindings::CallKind::Normal => Self::Normal,
            bindings::CallKind::Eager => Self::Eager,
            bindings::CallKind::Lazy => Self::Lazy,
        }
    }
}

impl From<CallKind> for bindings::CallKind {
    fn from(kind: CallKind) -> Self {
        match kind {
            CallKind::Normal => Self::Normal,
            CallKind::Eager => Self::Eager,
            CallKind::Lazy => Self::Lazy,
        }
    }
}

// ID

impl From<bindings::Id> for ID {
    fn from(id: bindings::Id) -> Self {
        let directory = id.directory.into_iter().map(|segment| segment.into()).collect();
        Self::new_for(id.kind.into(), directory, id.name.into())
    }
}

impl From<ID> for bindings::Id {
    fn from(id: ID) -> Self {
        let directory = id.directory.into_iter().map(|segment| segment.into()).collect();
        Self { kind: id.kind.into(), directory, name: id.name.into() }
    }
}
