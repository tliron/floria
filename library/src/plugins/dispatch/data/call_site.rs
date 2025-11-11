use super::super::super::super::data::*;

pub use super::super::super::bindings::exports::floria::plugins::dispatch::CallSite;

use {
    depiction::*,
    std::{fmt, io},
};

impl CallSite {
    /// Constructor.
    pub fn new(id: ID, property: Option<String>) -> Self {
        Self { id: id.into(), property }
    }
}

impl Depict for CallSite {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;

        let id: ID = self.id.clone().try_into().map_err(io::Error::other)?;
        id.kind.depict(writer, context)?;
        id.depict(writer, &context.child().with_separator(true))?;

        if let Some(property) = &self.property {
            context.theme.write_meta(writer, property)?;
        } else {
            context.theme.write_meta(writer, "no property")?;
        }

        Ok(())
    }
}

impl fmt::Display for CallSite {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id: ID = self.id.clone().try_into().map_err(|_| fmt::Error)?;
        fmt::Display::fmt(&id, formatter)?;
        if let Some(property) = &self.property {
            write!(formatter, ".{}", property)?;
        }
        Ok(())
    }
}
