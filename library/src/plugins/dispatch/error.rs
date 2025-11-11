use super::super::{super::data::*, bindings::exports::floria::plugins::dispatch};

use {
    depiction::{markup::*, *},
    derive_more::*,
    problemo::{common::*, *},
    std::{fmt, io},
};

//
// DispatchError
//

/// Dispatch error.
#[derive(Debug, Error, PartialEq)]
pub struct DispatchError {
    /// Message. Expects depiction markup.
    pub message_depiction_markup: String,

    /// Call.
    pub call: Call,

    /// Call site.
    pub call_site: dispatch::CallSite,
}

impl DispatchError {
    /// Constructor.
    pub fn new(message_depiction_markup: String, call: Call, call_site: dispatch::CallSite) -> Self {
        Self { message_depiction_markup, call, call_site }
    }

    /// Constructor.
    #[track_caller]
    pub fn as_problem(message_depiction_markup: String, call: Call, call_site: dispatch::CallSite) -> Problem {
        Self::new(message_depiction_markup, call, call_site)
            .into_problem()
            .with(CauseEquality::new::<Self>())
            .with(ErrorDepiction::new::<Self>())
            .with(ErrorMarkupDepicter::new::<Self>())
    }

    /// ID.
    pub fn id(&self) -> Result<ID, MalformedError> {
        self.call_site.id.clone().try_into()
    }
}

impl ToDepictionMarkup for DispatchError {
    fn to_depiction_markup(&self) -> String {
        format!("{} during {}", self.message_depiction_markup, self.call.to_depiction_markup())
    }

    fn into_depiction_markup(self) -> String {
        self.to_depiction_markup()
    }
}

impl Depict for DispatchError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;

        let id: ID = self.call_site.id.clone().try_into().map_err(io::Error::other)?;
        id.kind.depict(writer, context)?;
        context.child().with_separator(true).separate(writer)?;
        id.depict(writer, context)?;

        if let Some(property) = &self.call_site.property {
            context.indent(writer)?;
            context.theme.write_meta(writer, property)?;
        }

        context.indent_into_branch(writer, true)?;
        context.theme.write_depiction_markup(writer, &self.message_depiction_markup)?;

        let child_context = context.child().increase_indentation();
        child_context.indent(writer)?;
        write!(writer, "during ")?;
        self.call.depict(writer, context)
    }
}

impl fmt::Display for DispatchError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{} during {} at {}",
            escape_depiction_markup(&self.message_depiction_markup),
            self.call,
            &self.call_site
        )
    }
}
