use super::super::super::data::*;

use {
    depiction::*,
    derive_more::*,
    problemo::*,
    std::{fmt, io},
};

//
// WrongKindError
//

/// Wrong kind error.
#[derive(Debug, Error)]
pub struct WrongKindError {
    /// Expected kind.
    pub expected: EntityKind,

    /// Actual kind.
    pub actual: EntityKind,
}

impl WrongKindError {
    /// Constructor.
    pub fn new(expected: EntityKind, actual: EntityKind) -> Self {
        Self { expected, actual }
    }

    /// Constructor.
    pub fn problem(expected: EntityKind, actual: EntityKind) -> Problem {
        Self::new(expected, actual).into_problem().with(ErrorDepiction::new::<Self>())
    }
}

impl Depict for WrongKindError {
    fn depict<WriteT>(&self, _writer: &mut WriteT, _context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        todo!()
    }
}

impl fmt::Display for WrongKindError {
    fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}
