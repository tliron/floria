use super::{
    super::{
        super::{super::errors::MalformedError, function::*, id::*},
        expression::*,
    },
    kind::*,
};

use {
    compris::normal::*,
    depiction::*,
    kutil::std::{immutable::*, iter::*},
    std::{fmt, io},
};

//
// Call
//

/// Call.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Call {
    /// Function.
    pub function: FunctionName,

    /// Arguments.
    pub arguments: Vec<Expression>,

    /// Kind.
    pub kind: CallKind,
}

impl Call {
    /// Constructor.
    pub fn new(
        plugin_id: ID,
        function: ByteString,
        arguments: Vec<Expression>,
        kind: CallKind,
    ) -> Result<Self, MalformedError> {
        Ok(Self { function: FunctionName::new(plugin_id, function)?, arguments, kind })
    }
}

impl IntoDepictionMarkup for Call {
    fn into_depiction_markup(self) -> String {
        let arguments: Vec<String> =
            self.arguments.into_iter().map(|argument| argument.into_depiction_markup()).collect();
        format!(
            "{}|delimiter|(|{}|delimiter|)|",
            self.function.into_depiction_markup(),
            arguments.join("|delimiter|,|")
        )
    }
}

impl Depict for Call {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;

        match self.kind {
            CallKind::Eager => context.theme.write_delimiter(writer, '*')?,
            CallKind::Lazy => context.theme.write_delimiter(writer, '&')?,
            _ => {}
        }

        self.function.depict(writer, context)?;
        context.theme.write_delimiter(writer, '(')?;

        let child_context = &context.child().with_format(DepictionFormat::Compact).with_separator(false);
        for (argument, last) in IterateWithLast::new(&self.arguments) {
            argument.depict(writer, child_context)?;
            if !last {
                context.theme.write_delimiter(writer, ',')?;
            }
        }

        context.theme.write_delimiter(writer, ')')
    }
}

impl fmt::Display for Call {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            CallKind::Eager => write!(formatter, "*")?,
            CallKind::Lazy => write!(formatter, "&")?,
            _ => {}
        }

        write!(formatter, "{}(", self.function)?;

        for (argument, last) in IterateWithLast::new(&self.arguments) {
            fmt::Display::fmt(argument, formatter)?;
            if !last {
                write!(formatter, ",")?;
            }
        }

        write!(formatter, ")")
    }
}

// Conversions

impl<AnnotatedT> Into<Variant<AnnotatedT>> for Call
where
    AnnotatedT: Default,
{
    fn into(self) -> Variant<AnnotatedT> {
        let mut map = Map::default();

        map.into_insert("$plugin", self.function.plugin_id.to_string());
        map.into_insert("$function", self.function.name);

        if !self.arguments.is_empty() {
            let arguments: List<AnnotatedT> = self.arguments.into_iter().map(|argument| argument.into()).collect();
            map.into_insert("$arguments", arguments);
        }

        match self.kind {
            CallKind::Eager => {
                map.into_insert("$kind", "eager");
            }
            CallKind::Lazy => {
                map.into_insert("$kind", "lazy");
            }
            _ => {}
        }

        map.into()
    }
}
