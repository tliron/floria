use super::super::super::super::data::*;

pub use super::super::super::bindings::exports::floria::plugins::dispatch::CallSite;

use {
    kutil::{cli::depict::*, std::iter::*},
    std::{fmt, io},
};

impl CallSite {
    /// Constructor.
    pub fn new(id: ID, path: Vec<String>) -> Self {
        Self { id: id.into(), path }
    }
}

impl Depict for CallSite {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;

        let id: ID = self.id.clone().into();
        id.kind.depict(writer, context)?;
        id.depict(writer, &context.child().with_separator(true))?;

        if !self.path.is_empty() {
            for (segment, last) in IterateWithLast::new(&self.path) {
                context.theme.write_meta(writer, segment)?;
                if !last {
                    context.theme.write_delimiter(writer, '.')?;
                }
            }
        } else {
            context.theme.write_meta(writer, "no path")?;
        }

        Ok(())
    }
}

impl fmt::Display for CallSite {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id: ID = self.id.clone().into();
        fmt::Display::fmt(&id, formatter)?;
        for segment in &self.path {
            write!(formatter, ".{}", segment)?;
        }
        Ok(())
    }
}
