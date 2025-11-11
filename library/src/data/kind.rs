use {depiction::*, kutil::std::*, std::io};

//
// EntityKind
//

/// Entity kind.
#[derive(Clone, Copy, Debug, Default, Display, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd)]
pub enum EntityKind {
    /// Plugin.
    #[default]
    Plugin,

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
