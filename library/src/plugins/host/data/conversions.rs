use super::super::super::{
    super::{data::*, errors::*},
    bindings::floria::plugins::floria as bindings,
    dispatch::*,
};

// Kind

impl From<bindings::EntityKind> for EntityKind {
    fn from(kind: bindings::EntityKind) -> Self {
        match kind {
            bindings::EntityKind::Plugin => Self::Plugin,
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
            EntityKind::Plugin => Self::Plugin,
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

impl TryFrom<bindings::Id> for ID {
    type Error = MalformedError;

    fn try_from(id: bindings::Id) -> Result<Self, Self::Error> {
        let directory = Directory::new(id.directory.into_iter().map(|segment| segment.into()).collect())?;
        Self::new_with_name(id.kind.into(), directory, id.name.into())
    }
}

impl From<ID> for bindings::Id {
    fn from(id: ID) -> Self {
        let directory = id.directory.into_iter().map(|segment| segment.into()).collect();
        Self { kind: id.kind.into(), directory, name: id.name.into() }
    }
}

/// Convert ID option to bindings.
pub fn id_option_to_bindings(id: Option<ID>) -> Option<bindings::Id> {
    id.map(|id| id.into())
}

/// Convert ID option from bindings.
pub fn id_option_from_bindings(id: Option<bindings::Id>) -> Result<Option<ID>, MalformedError> {
    Ok(match id {
        Some(id) => Some(id.try_into()?),
        None => None,
    })
}

/// Convert IDs to bindings.
pub fn ids_to_bindings(ids: Vec<ID>) -> Vec<bindings::Id> {
    ids.into_iter().map(|id| id.into()).collect()
}

/// Convert IDs from bindings.
pub fn ids_from_bindings(ids: Vec<bindings::Id>) -> Result<Vec<ID>, MalformedError> {
    ids.into_iter().map(|id| id.try_into()).collect()
}

// CallSite

impl TryFrom<bindings::CallSite> for CallSite {
    type Error = MalformedError;

    fn try_from(call_site: bindings::CallSite) -> Result<Self, Self::Error> {
        Ok(Self::new(call_site.id.try_into()?, call_site.property))
    }
}

impl TryFrom<CallSite> for bindings::CallSite {
    type Error = MalformedError;

    fn try_from(call_site: CallSite) -> Result<Self, Self::Error> {
        let id: ID = call_site.id.try_into()?;
        Ok(Self { id: id.into(), property: call_site.property })
    }
}
