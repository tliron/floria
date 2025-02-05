use super::super::expression::*;

use {
    compris::normal::*,
    kutil::{
        cli::depict::*,
        std::{immutable::*, iter::*},
    },
    std::{fmt, io},
};

//
// Call
//

/// Call.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Call {
    /// Plugin name.
    pub plugin: ByteString,

    /// Function name.
    pub function: ByteString,

    /// Arguments.
    pub arguments: Vec<Expression>,

    /// True if eagerly evaluated.
    pub eager: bool,
}

impl Call {
    /// Constructor.
    pub fn new(plugin: ByteString, function: ByteString, arguments: Vec<Expression>, eager: bool) -> Self {
        Self { plugin, function, arguments, eager }
    }
}

impl Depict for Call {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        if self.eager {
            context.theme.write_delimiter(writer, '*')?;
        }
        context.theme.write_name(writer, &self.plugin)?;
        context.theme.write_delimiter(writer, ':')?;
        context.theme.write_name(writer, &self.function)?;
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
        if self.eager {
            write!(formatter, "*")?;
        }

        write!(formatter, "{}:{}(", self.plugin, self.function)?;

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

        map.into_insert("$plugin", self.plugin);
        map.into_insert("$function", self.function);

        if !self.arguments.is_empty() {
            let arguments: List<AnnotatedT> = self.arguments.into_iter().map(|argument| argument.into()).collect();
            map.into_insert("$arguments", arguments);
        }

        if self.eager {
            map.into_insert("$eager", true);
        }

        map.into()
    }
}
