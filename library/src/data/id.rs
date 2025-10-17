use super::{directory::*, kind::*};

use {
    depiction::*,
    kutil::std::immutable::*,
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
        match id.split_once(":") {
            Some((directory, name)) => {
                let Ok(directory) = directory.parse();
                Self::new_for(kind, directory, name.into())
            }

            None => Self::new_for(kind, Default::default(), id.into()),
        }
    }
}

impl Depict for ID {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        if self.directory.0.is_empty() {
            context.separate(writer)?;
        } else {
            self.directory.depict(writer, context)?;
            context.theme.write_delimiter(writer, ':')?;
        }
        write!(writer, "{}", context.theme.name_style.bold().style(&self.name))
    }
}

impl fmt::Display for ID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.directory.0.is_empty() {
            fmt::Display::fmt(&self.name, formatter)
        } else {
            write!(formatter, "{}:{}", self.directory, self.name)
        }
    }
}
