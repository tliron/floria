use super::{
    super::{data::*, utils::*, *},
    instance::*,
};

impl floria_bindings::Entity {
    /// Property.
    pub fn property(&self, name: &str) -> Option<&floria_bindings::Property> {
        match self {
            Self::Vertex(vertex) => vertex.property(name),
            Self::Edge(edge) => edge.property(name),
        }
    }

    /// Property.
    pub fn must_property(&self, name: &str) -> Result<&floria_bindings::Property, DispatchError> {
        self.property(name).ok_or_else(|| format!("missing |meta|property|: |error|{}|", escape_depiction_markup(name)))
    }

    /// Kind.
    pub fn kind(&self) -> EntityKind {
        match self {
            Self::Vertex(_) => EntityKind::Vertex,
            Self::Edge(_) => EntityKind::Edge,
        }
    }

    /// Error if not of the kind.
    pub fn assert_kind(&self, kind: EntityKind) -> Result<(), DispatchError> {
        if self.kind() == kind {
            Ok(())
        } else {
            Err(format!("not {} |meta|{}|", kind.article(), escape_depiction_markup(kind)))
        }
    }
}

// Conversion

impl From<floria_bindings::Vertex> for floria_bindings::Entity {
    fn from(vertex: floria_bindings::Vertex) -> Self {
        Self::Vertex(vertex)
    }
}

impl From<floria_bindings::Edge> for floria_bindings::Entity {
    fn from(edge: floria_bindings::Edge) -> Self {
        Self::Edge(edge)
    }
}

impl TryInto<floria_bindings::Vertex> for floria_bindings::Entity {
    type Error = DispatchError;

    fn try_into(self) -> Result<floria_bindings::Vertex, Self::Error> {
        match self {
            Self::Vertex(vertex) => Ok(vertex),
            _ => Err(format!("not a |meta|vertex|: |error|{}|", self.kind())),
        }
    }
}

impl TryInto<floria_bindings::Edge> for floria_bindings::Entity {
    type Error = DispatchError;

    fn try_into(self) -> Result<floria_bindings::Edge, Self::Error> {
        match self {
            Self::Edge(edge) => Ok(edge),
            _ => Err(format!("not an |meta|edge|: |error|{}|", self.kind())),
        }
    }
}
