use super::super::data::*;

use std::fmt;

//
// Id
//

impl Id {
    /// Constructor
    pub fn new(kind: EntityKind, directory: Directory, name: String) -> Self {
        Self { kind, directory, name }
    }

    /// Parse.
    pub fn parse(kind: EntityKind, id: &str) -> Self {
        match id.split_once(":") {
            Some((directory, name)) => {
                let directory = directory.split('/').map(|segment| segment.into()).collect();
                Self::new(kind, directory, name.into())
            }

            None => Self::new(kind, Default::default(), id.into()),
        }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.directory.is_empty() {
            write!(formatter, "{}:", self.directory.join("/"))?;
        }
        fmt::Display::fmt(&self.name, formatter)
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        (self.kind == other.kind) && (self.directory == other.directory) && (self.name == other.name)
    }
}

impl Eq for Id {}
