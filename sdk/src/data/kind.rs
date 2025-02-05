use super::super::data::*;

use std::fmt;

impl EntityKind {
    /// As string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Class => "Class",
            Self::VertexTemplate => "VertexTemplate",
            Self::EdgeTemplate => "EdgeTemplate",
            Self::Vertex => "Vertex",
            Self::Edge => "Edge",
        }
    }

    /// Article ("a" or "an").
    pub fn article(&self) -> &'static str {
        match self {
            Self::EdgeTemplate | Self::Edge => "an",
            _ => "a",
        }
    }
}

impl fmt::Display for EntityKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), formatter)
    }
}

impl PartialEq for EntityKind {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Class, Self::Class)
                | (Self::VertexTemplate, Self::VertexTemplate)
                | (Self::EdgeTemplate, Self::EdgeTemplate)
                | (Self::Vertex, Self::Vertex)
                | (Self::Edge, Self::Edge)
        )
    }
}

impl Into<&'static str> for &EntityKind {
    fn into(self) -> &'static str {
        self.as_str()
    }
}

impl TryFrom<&str> for EntityKind {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Class" => Ok(Self::Class),
            "VertexTemplate" => Ok(Self::VertexTemplate),
            "EdgeTemplate" => Ok(Self::EdgeTemplate),
            "Vertex" => Ok(Self::Vertex),
            "Edge" => Ok(Self::Edge),
            _ => Err(format!("unsupported entity kind: {}", value)),
        }
    }
}
