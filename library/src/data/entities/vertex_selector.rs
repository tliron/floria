use super::{
    super::{expression::*, id::*},
    vertex_finder::*,
};

use {
    kutil::cli::depict::*,
    std::{collections::*, io},
};

//
// VertexSelector
//

/// Vertex selector.
#[derive(Clone, Debug)]
pub enum VertexSelector {
    /// Vertex ID.
    VertexID(ID),

    /// Finder
    Finder(VertexFinder),
}

impl VertexSelector {
    /// Constructor.
    pub fn new_vertex(vertex_id: ID) -> Self {
        Self::VertexID(vertex_id)
    }

    /// Constructor.
    pub fn new_finder(finder: Call) -> Self {
        Self::Finder(VertexFinder::new(finder))
    }
}

impl Depict for VertexSelector {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::VertexID(id) => id.depict(writer, context),
            Self::Finder(vertex_filter) => vertex_filter.depict(writer, context),
        }
    }
}

impl Into<Expression> for VertexSelector {
    fn into(self) -> Expression {
        let mut map = BTreeMap::default();

        match self {
            Self::VertexID(id) => {
                map.insert("id".into(), id.kind.as_str().into());
            }

            Self::Finder(vertex_finder) => {
                map.insert("finder".into(), vertex_finder.into());
            }
        }

        Expression::Map(map)
    }
}
