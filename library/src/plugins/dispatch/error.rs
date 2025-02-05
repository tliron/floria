use super::super::{super::data::*, bindings::exports::floria::plugins::dispatch};

use {
    kutil::cli::depict::*,
    std::{error::*, fmt, io},
};

//
// DispatchError
//

/// Dispatch error.
#[derive(Debug)]
pub struct DispatchError {
    /// Message. Expects depiction markup.
    pub message: String,

    /// Call.
    pub call: Call,

    /// Call site.
    pub call_site: dispatch::CallSite,
}

impl DispatchError {
    /// Constructor.
    pub fn new(message: String, call: Call, call_site: dispatch::CallSite) -> Self {
        Self { message, call, call_site }
    }

    /// ID.
    pub fn id(&self) -> ID {
        self.call_site.id.clone().into()
    }
}

impl Depict for DispatchError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;

        if let Some(property) = &self.call_site.property {
            context.theme.write_meta(writer, property)?;
        } else {
            context.theme.write_meta(writer, "no property")?;
        }

        context.indent_into_branch(writer, true)?;
        context.theme.write_depiction_markup(writer, &self.message)?;

        context.child().increase_indentation().indent(writer)?;
        write!(writer, "during ")?;
        self.call.depict(writer, context)
    }
}

impl fmt::Display for DispatchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} during {} at {}", escape_depiction_markup(&self.message), self.call, &self.call_site)
    }
}

impl Error for DispatchError {}
