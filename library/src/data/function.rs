use super::{super::errors::*, id::*};

use {
    depiction::*,
    kutil::std::immutable::*,
    std::{fmt, io},
};

/// Function delimiter.
pub const FUNCTION_DELIMITER: char = ':';

/// Invalid name characters.
pub const INVALID_FUNCTION_CHARACTERS: [char; 1] = [FUNCTION_DELIMITER];

//
// FunctionName
//

/// Function name.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FunctionName {
    /// Plugin ID.
    pub plugin_id: ID,

    /// Function name.
    pub name: ByteString,
}

impl FunctionName {
    /// Constructor.
    pub fn new(plugin_id: ID, name: ByteString) -> Result<Self, MalformedError> {
        for c in INVALID_FUNCTION_CHARACTERS {
            if name.contains(c) {
                return Err(format!("function name contains invalid character: {}", c).into());
            }
        }

        Ok(Self { plugin_id, name })
    }
}

impl IntoDepictionMarkup for FunctionName {
    fn into_depiction_markup(self) -> String {
        format!(
            "{}|delimiter|{}||name|{}|",
            self.plugin_id.into_depiction_markup(),
            FUNCTION_DELIMITER,
            escape_depiction_markup(self.name)
        )
    }
}

impl Depict for FunctionName {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        self.plugin_id.depict(writer, context)?;
        context.theme.write_delimiter(writer, FUNCTION_DELIMITER)?;
        context.theme.write_name(writer, &self.name)
    }
}

impl fmt::Display for FunctionName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.plugin_id, self.name)
    }
}
