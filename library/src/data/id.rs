use super::{directory::*, kind::*};

use {
    kutil::{cli::depict::*, std::immutable::*},
    std::{fmt, io},
};

//
// ID
//

/// ID.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ID {
    /// Kind.
    pub kind: EntityKind,

    /// Directory.
    pub directory: Directory,

    /// Name.
    pub name: ByteString,
}

impl ID {
    /// Constructor
    pub fn new(kind: EntityKind, directory: Directory) -> Self {
        Self::new_for(kind, directory, Default::default())
    }

    /// Constructor
    pub fn new_for(kind: EntityKind, directory: Directory, name: ByteString) -> Self {
        Self { kind, directory, name }
    }

    /// Parse.
    pub fn parse(kind: EntityKind, id: &str) -> Self {
        let segments: Vec<&str> = id.split(":").collect();
        let length = segments.len();
        if length > 0 {
            Self::new_for(
                kind,
                segments[..length - 1].iter().map(|segment| (*segment).into()).collect(),
                segments[length - 1].into(),
            )
        } else {
            Self::new_for(kind, Default::default(), id.into())
        }
    }

    /// Parse [Directory].
    pub fn parse_directory(directory: &str) -> Directory {
        directory.split(":").map(|segment| segment.into()).collect()
    }

    /// To [Directory].
    pub fn to_directory(&self) -> Directory {
        let mut directory = self.directory.clone();
        directory.add_last_segment(self.name.clone());
        directory
    }
}

impl Depict for ID {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        for segment in &self.directory {
            write!(writer, "{}", context.theme.name_style.remove_all_effects().style(segment))?;
            context.theme.write_delimiter(writer, ':')?;
        }
        write!(writer, "{}", context.theme.name_style.bold().style(&self.name))
    }
}

impl fmt::Display for ID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.directory {
            write!(formatter, "{}:", segment)?;
        }
        write!(formatter, "{}", self.name)
    }
}
