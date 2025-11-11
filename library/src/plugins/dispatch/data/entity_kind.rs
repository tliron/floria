use super::super::super::bindings::exports::floria::plugins::dispatch::*;

use std::{cmp::*, mem::*};

impl PartialEq for EntityKind {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for EntityKind {}

impl PartialOrd for EntityKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EntityKind {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (EntityKind::Plugin, EntityKind::Plugin)
            | (EntityKind::Class, EntityKind::Class)
            | (EntityKind::VertexTemplate, EntityKind::VertexTemplate)
            | (EntityKind::EdgeTemplate, EntityKind::EdgeTemplate)
            | (EntityKind::Vertex, EntityKind::Vertex)
            | (EntityKind::Edge, EntityKind::Edge) => Ordering::Equal,

            (EntityKind::Plugin, _) => Ordering::Less,

            (EntityKind::Class, EntityKind::Plugin) => Ordering::Greater,
            (EntityKind::Class, _) => Ordering::Less,

            (EntityKind::VertexTemplate, EntityKind::Plugin) | (EntityKind::VertexTemplate, EntityKind::Class) => {
                Ordering::Greater
            }
            (EntityKind::VertexTemplate, _) => Ordering::Less,

            (EntityKind::EdgeTemplate, EntityKind::Plugin)
            | (EntityKind::EdgeTemplate, EntityKind::Class)
            | (EntityKind::EdgeTemplate, EntityKind::VertexTemplate) => Ordering::Greater,
            (EntityKind::EdgeTemplate, _) => Ordering::Less,

            (EntityKind::Vertex, EntityKind::Plugin)
            | (EntityKind::Vertex, EntityKind::Class)
            | (EntityKind::Vertex, EntityKind::VertexTemplate)
            | (EntityKind::Vertex, EntityKind::EdgeTemplate) => Ordering::Greater,
            (EntityKind::Vertex, _) => Ordering::Less,

            (EntityKind::Edge, _) => Ordering::Greater,
        }
    }
}
