use super::super::{super::data::*, bindings::exports::floria::plugins::dispatch};

use {
    kutil::{cli::depict::*, std::iter::*},
    std::{error::*, fmt, io},
};

//
// DispatchError
//

/// Dispatch error.
#[derive(Debug)]
pub struct DispatchError {
    /// Message.
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

        if !self.call_site.path.is_empty() {
            for (segment, last) in IterateWithLast::new(&self.call_site.path) {
                context.theme.write_meta(writer, segment)?;
                if !last {
                    context.theme.write_delimiter(writer, '.')?;
                }
            }
        } else {
            context.theme.write_meta(writer, "no path")?;
        }

        context.indent_into_branch(writer, false)?;
        self.call.depict(writer, context)?;

        context.indent_into_branch(writer, true)?;
        context.theme.write_error(writer, &self.message)
    }
}

impl fmt::Display for DispatchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} during {} at {}", self.message, self.call, &self.call_site)
    }
}

impl Error for DispatchError {}
