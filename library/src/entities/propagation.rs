use super::super::data::*;

use kutil::std::collections::*;

//
// Propagation
//

/// Propagation.
#[derive(Clone, Debug, Default)]
pub struct Propagation {
    /// Into vertex's containing vertex.
    pub containing_vertex: bool,

    /// Into vertex's contained vertexes.
    pub contained_vertexes: bool,

    /// Into incoming edges.
    pub incoming_edges: bool,

    /// Into outgoing edges.
    pub outgoing_edges: bool,

    /// Into edge's source vertex.
    pub source_vertex: bool,

    /// Into edge's target vertex.
    pub target_vertex: bool,

    /// Propagated IDs.
    pub propagated: FastHashSet<ID>,
}

impl Propagation {
    /// Outgoing all.
    pub fn outgoing_all() -> Self {
        Self {
            containing_vertex: true,
            contained_vertexes: true,
            incoming_edges: false,
            outgoing_edges: true,
            source_vertex: false,
            target_vertex: true,
            propagated: Default::default(),
        }
    }

    /// True if we should propagate into ID.
    ///
    /// Will add the ID to avoid propagating into it again.
    pub fn should(&mut self, id: &ID) -> bool {
        if self.propagated.contains(id) {
            false
        } else {
            self.propagated.insert(id.clone());
            true
        }
    }
}
