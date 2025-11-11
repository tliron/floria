use super::super::super::super::data::*;

pub use super::super::super::bindings::exports::floria::plugins::dispatch::CallSite;

use {
    depiction::*,
    std::{cmp::*, fmt, io},
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
        context.separate(writer)?;
        id.depict(writer, context)?;

        if let Some(property) = &self.property {
            context.separate(writer)?;
            context.theme.write_meta(writer, property)?;
        }

        Ok(())
    }
}

impl fmt::Display for CallSite {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let id: ID = self.id.clone().try_into().map_err(|_| fmt::Error)?;
        fmt::Display::fmt(&id, formatter)?;
        if let Some(property) = &self.property {
            write!(formatter, ".{}", property)?;
        }
        Ok(())
    }
}

impl PartialEq for CallSite {
    fn eq(&self, other: &Self) -> bool {
        (self.id == other.id) && (self.property == other.property)
    }
}

impl Eq for CallSite {}

impl PartialOrd for CallSite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CallSite {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.id.cmp(&other.id) {
            Ordering::Equal => {}
            ordering => return ordering,
        }

        self.property.cmp(&other.property)
    }
}
