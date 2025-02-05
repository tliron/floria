use {
    kutil::{cli::depict::*, std::*},
    std::io,
};

//
// EntityKind
//

/// Entity kind.
#[derive(Clone, Copy, Debug, Display, Eq, FromStr, Hash, PartialEq)]
pub enum EntityKind {
    /// Class.
    Class,

    /// Vertex template.
    VertexTemplate,

    /// Edge template.
    EdgeTemplate,

    /// Vertex.
    Vertex,

    /// Edge.
    Edge,
}

impl Depict for EntityKind {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_meta(writer, self)
    }
}
