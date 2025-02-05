use super::{super::dispatch_bindings::*, directory::*};

use std::fmt;

//
// Id
//

impl Id {
    /// Constructor
    pub fn new(kind: Kind, directory: Directory, id: String) -> Self {
        Self { kind, directory, id }
    }

    /// Constructor
    pub fn new_from(expression: &Expression) -> Option<Self> {
        if let Some(id) = expression.get(&"id".into())
            && let Expression::Text(id) = id
            && let Some(kind) = expression.get(&"kind".into())
            && let Expression::Text(kind) = kind
            && let Ok(kind) = Kind::try_from(kind.as_str())
        {
            return Some(Self::parse(kind, id));
        }

        None
    }

    /// Parse.
    pub fn parse(kind: Kind, id: &str) -> Self {
        let segments: Vec<&str> = id.split(":").collect();
        let length = segments.len();
        if length > 0 {
            Self::new(
                kind,
                segments[..length - 1].iter().map(|segment| segment.to_string()).collect(),
                segments[length - 1].into(),
            )
        } else {
            Self::new(kind, Default::default(), id.into())
        }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.directory {
            write!(formatter, "{}:", segment)?;
        }
        write!(formatter, "{}", self.id)
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        (self.kind == other.kind) && (self.directory == other.directory) && (self.id == other.id)
    }
}
