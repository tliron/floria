use super::{super::errors::*, directory::*, kind::*};

use {
    depiction::*,
    kutil::std::immutable::*,
    std::{fmt, io},
};

/// Invalid name characters.
pub const INVALID_NAME_CHARACTERS: [char; 1] = [DIRECTORY_DELIMITER];

//
// ID
//

/// ID.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
        Self { kind, directory, name: Default::default() }
    }

    /// Constructor
    pub fn new_for(kind: EntityKind, directory: Directory, name: ByteString) -> Result<Self, MalformedError> {
        for c in INVALID_NAME_CHARACTERS {
            if name.contains(c) {
                return Err(format!("ID name contains invalid character: {}", c).into());
            }
        }

        Ok(Self { kind, directory, name })
    }

    /// Parse.
    pub fn parse(kind: EntityKind, id: &str) -> Result<Self, MalformedError> {
        match id.rsplit_once("/") {
            Some((directory, name)) => Self::new_for(kind, directory.parse()?, name.into()),
            None => Self::new_for(kind, Default::default(), id.into()),
        }
    }
}

impl IntoDepictionMarkup for ID {
    fn into_depiction_markup(self) -> String {
        if self.directory.is_empty() {
            format!("|name|{}|", escape_depiction_markup(self.name))
        } else {
            format!(
                "{}|delimiter|{}||name|{}|",
                self.directory.into_depiction_markup(),
                DIRECTORY_DELIMITER,
                escape_depiction_markup(self.name)
            )
        }
    }
}

impl Depict for ID {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        if self.directory.is_empty() {
            context.separate(writer)?;
        } else {
            self.directory.depict(writer, context)?;
            context.theme.write_delimiter(writer, DIRECTORY_DELIMITER)?;
        }
        context.theme.write_name(writer, &self.name)
    }
}

impl fmt::Display for ID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.directory.is_empty() {
            write!(formatter, "{}", self.name)
        } else {
            write!(formatter, "{}{}{}", self.directory, DIRECTORY_DELIMITER, self.name)
        }
    }
}
