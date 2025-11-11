use super::super::super::super::bindings::floria::plugins::floria as bindings;

// impl From<bindings::Class> for bindings::Entity {
//     fn from(class: bindings::Class) -> Self {
//         bindings::Entity::Class(class)
//     }
// }

// impl From<bindings::VertexTemplate> for bindings::Entity {
//     fn from(vertex_template: bindings::VertexTemplate) -> Self {
//         bindings::Entity::VertexTemplate(vertex_template)
//     }
// }

// impl From<bindings::EdgeTemplate> for bindings::Entity {
//     fn from(edge_template: bindings::EdgeTemplate) -> Self {
//         bindings::Entity::EdgeTemplate(edge_template)
//     }
// }

impl From<bindings::Vertex> for bindings::Entity {
    fn from(vertex: bindings::Vertex) -> Self {
        bindings::Entity::Vertex(vertex)
    }
}

impl From<bindings::Edge> for bindings::Entity {
    fn from(edge: bindings::Edge) -> Self {
        bindings::Entity::Edge(edge)
    }
}
