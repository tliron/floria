use {
    kutil::cli::depict::*,
    std::{error::*, fmt, io},
};

//
// InitializationError
//

/// Initialization error.
#[derive(Debug)]
pub struct InitializationError {
    /// Message. Expects depiction markup.
    pub message: String,
}

impl InitializationError {
    /// Constructor.
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Depict for InitializationError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_depiction_markup(writer, &self.message)
    }
}

impl fmt::Display for InitializationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", escape_depiction_markup(&self.message))
    }
}

impl Error for InitializationError {}
