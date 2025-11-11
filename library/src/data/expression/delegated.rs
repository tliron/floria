use super::expression::*;

use {
    depiction::*,
    std::{cmp::*, fmt, hash::*, io},
};

impl IntoDepictionMarkup for Expression {
    fn into_depiction_markup(self) -> String {
        match self {
            Self::Undefined => "|symbol|Undefined|".into(),

            Self::Null => "|symbol|Null|".into(),

            Self::Integer(integer) => format!("|number|{:+}|", integer),

            Self::UnsignedInteger(unsigned_integer) => format!("|number|{}|", unsigned_integer),

            Self::Float(float) => format!("|number|{:?}|", float),

            Self::Boolean(boolean) => format!("|symbol|{}|", boolean),

            Self::Text(text) => format!("|string|{}|", escape_depiction_markup(format!("{:?}", text))),

            Self::Blob(blob) => format!("|number|{}| bytes", blob.len()),

            Self::List(list) => {
                let items: Vec<String> = list.into_iter().map(|item| item.into_depiction_markup()).collect();
                format!("|delimiter|[|{}|delimiter|]|", items.join("|delimiter|,|"))
            }

            Self::Map(map) => {
                let items: Vec<String> = map
                    .into_iter()
                    .map(|(key, value)| {
                        format!("{}|delimiter|:|{}", key.into_depiction_markup(), value.into_depiction_markup())
                    })
                    .collect();
                format!("|delimiter|{{|{}|delimiter|}}|", items.join("|delimiter|,|"))
            }

            Self::Custom(kind, inner) => {
                format!("|heading|Custom| |name|{}| {}", escape_depiction_markup(kind), inner.into_depiction_markup())
            }

            Self::Call(call) => call.into_depiction_markup(),
        }
    }
}

impl Depict for Expression {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Undefined => {
                context.separate(writer)?;
                context.theme.write_symbol(writer, "Undefined")
            }

            Self::Null => {
                context.separate(writer)?;
                context.theme.write_symbol(writer, "Null")
            }

            Self::Integer(integer) => {
                context.separate(writer)?;
                if context.get_format() == DepictionFormat::Compact {
                    write!(writer, "{}", context.theme.number(format!("{:+}", integer)))
                } else {
                    write!(writer, "{} {}", context.theme.number(integer), context.theme.meta("i64"))
                }
            }

            Self::UnsignedInteger(unsigned_integer) => {
                context.separate(writer)?;
                if context.get_format() == DepictionFormat::Compact {
                    context.theme.write_number(writer, unsigned_integer)
                } else {
                    write!(writer, "{} {}", context.theme.number(unsigned_integer), context.theme.meta("u64"))
                }
            }

            Self::Float(float) => {
                context.separate(writer)?;
                if context.get_format() == DepictionFormat::Compact {
                    write!(writer, "{}", context.theme.number(format!("{:?}", float)))
                } else {
                    write!(writer, "{} {}", context.theme.number(float), context.theme.meta("f64"))
                }
            }

            Self::Boolean(boolean) => {
                context.separate(writer)?;
                context.theme.write_symbol(writer, boolean)
            }

            Self::Text(text) => {
                context.separate(writer)?;
                write!(writer, "{}", context.theme.string(format!("{:?}", text)))
            }

            Self::Blob(blob) => {
                context.separate(writer)?;
                context.theme.write_symbol(writer, format!("{} bytes", blob.len()))
            }

            Self::List(list) => utils::depict_list(list.iter(), None, writer, context),

            Self::Map(map) => utils::depict_map(map.iter(), None, writer, context),

            Self::Custom(kind, inner) => {
                context.separate(writer)?;
                context.theme.write_heading(writer, "Custom")?;
                context.separate(writer)?;
                context.theme.write_name(writer, kind)?;
                inner.depict(writer, context)
            }

            Self::Call(call) => {
                context.separate(writer)?;
                call.depict(writer, &context.child().with_separator(false))
            }
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(
            &self
                .to_depiction(&PLAIN_DEPICTION_CONTEXT.child().with_format(DepictionFormat::Compact))
                .map_err(|_error| fmt::Error)?,
            formatter,
        )
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Undefined, Self::Undefined) | (Self::Null, Self::Null) => true,
            (Self::Integer(left), Self::Integer(right)) => left == right,
            (Self::UnsignedInteger(left), Self::UnsignedInteger(right)) => left == right,
            (Self::Float(left), Self::Float(right)) => left == right,
            (Self::Boolean(left), Self::Boolean(right)) => left == right,
            (Self::Text(left), Self::Text(right)) => left == right,
            (Self::Blob(left), Self::Blob(right)) => left == right,
            (Self::List(left), Self::List(right)) => left == right,
            (Self::Map(left), Self::Map(right)) => left == right,
            (Self::Custom(left_kind, left_properties), Self::Custom(right_kind, right_properties)) => {
                (left_kind == right_kind) && (left_properties == right_properties)
            }
            (Self::Call(left), Self::Call(right)) => left == right,
            _ => false,
        }
    }
}

impl Eq for Expression {}

impl PartialOrd for Expression {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Undefined, Self::Undefined) | (Self::Null, Self::Null) => Some(Ordering::Equal),
            (Self::Integer(left), Self::Integer(right)) => left.partial_cmp(right),
            (Self::UnsignedInteger(left), Self::UnsignedInteger(right)) => left.partial_cmp(right),
            (Self::Float(left), Self::Float(right)) => left.partial_cmp(right),
            (Self::Boolean(left), Self::Boolean(right)) => left.partial_cmp(right),
            (Self::Text(left), Self::Text(right)) => left.partial_cmp(right),
            (Self::Blob(left), Self::Blob(right)) => left.partial_cmp(right),
            (Self::List(left), Self::List(right)) => left.partial_cmp(right),
            (Self::Map(left), Self::Map(right)) => left.partial_cmp(right),
            (Self::Custom(left_kind, left_properties), Self::Custom(right_kind, right_properties)) => {
                match left_kind.partial_cmp(right_kind) {
                    Some(Ordering::Equal) => left_properties.partial_cmp(right_properties),
                    ordering => ordering,
                }
            }
            (Self::Call(left), Self::Call(right)) => left.partial_cmp(right),
            _ => None,
        }
    }
}

impl Ord for Expression {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Undefined, Self::Undefined) | (Self::Null, Self::Null) => Ordering::Equal,
            (Self::Integer(left), Self::Integer(right)) => left.cmp(right),
            (Self::UnsignedInteger(left), Self::UnsignedInteger(right)) => left.cmp(right),
            (Self::Float(left), Self::Float(right)) => left.cmp(right),
            (Self::Boolean(left), Self::Boolean(right)) => left.cmp(right),
            (Self::Text(left), Self::Text(right)) => left.cmp(right),
            (Self::Blob(left), Self::Blob(right)) => left.cmp(right),
            (Self::List(left), Self::List(right)) => left.cmp(right),
            (Self::Map(left), Self::Map(right)) => left.cmp(right),
            (Self::Custom(left_kind, left_properties), Self::Custom(right_kind, right_properties)) => {
                match left_kind.cmp(right_kind) {
                    Ordering::Equal => left_properties.cmp(right_properties),
                    ordering => ordering,
                }
            }
            (Self::Call(left), Self::Call(right)) => left.cmp(right),

            (Self::Undefined, _) => Ordering::Less,

            (Self::Null, Self::Undefined) => Ordering::Greater,
            (Self::Null, _) => Ordering::Less,

            (Self::Integer(_), Self::Undefined | Self::Null) => Ordering::Greater,
            (Self::Integer(_), _) => Ordering::Less,

            (Self::UnsignedInteger(_), Self::Undefined | Self::Null | Self::Integer(_)) => Ordering::Greater,
            (Self::UnsignedInteger(_), _) => Ordering::Less,

            (Self::Float(_), Self::Undefined | Self::Null | Self::Integer(_) | Self::UnsignedInteger(_)) => {
                Ordering::Greater
            }
            (Self::Float(_), _) => Ordering::Less,

            (
                Self::Boolean(_),
                Self::Undefined | Self::Null | Self::Integer(_) | Self::UnsignedInteger(_) | Self::Float(_),
            ) => Ordering::Greater,
            (Self::Boolean(_), _) => Ordering::Less,

            (
                Self::Text(_),
                Self::Undefined
                | Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_),
            ) => Ordering::Greater,
            (Self::Text(_), _) => Ordering::Less,

            (
                Self::Blob(_),
                Self::Undefined
                | Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_),
            ) => Ordering::Greater,
            (Self::Blob(_), _) => Ordering::Less,

            (
                Self::List(_),
                Self::Undefined
                | Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_)
                | Self::Blob(_),
            ) => Ordering::Greater,
            (Self::List(_), _) => Ordering::Less,

            (
                Self::Map(_),
                Self::Undefined
                | Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_)
                | Self::Blob(_)
                | Self::List(_),
            ) => Ordering::Greater,
            (Self::Map(_), _) => Ordering::Less,

            (
                Self::Custom(_, _),
                Self::Undefined
                | Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_)
                | Self::Blob(_)
                | Self::List(_)
                | Self::Map(_),
            ) => Ordering::Greater,
            (Self::Custom(_, _), _) => Ordering::Less,

            (Self::Call(_), _) => Ordering::Less,
        }
    }
}

impl Hash for Expression {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        match self {
            Self::Undefined => {
                state.write_u8(1);
            }

            Self::Null => {
                state.write_u8(2);
            }

            Self::Integer(integer) => {
                state.write_u8(3);
                integer.hash(state);
            }

            Self::UnsignedInteger(unsigned_integer) => {
                state.write_u8(4);
                unsigned_integer.hash(state);
            }

            Self::Float(float) => {
                state.write_u8(5);
                float.hash(state);
            }

            Self::Boolean(boolean) => {
                state.write_u8(6);
                boolean.hash(state);
            }

            Self::Text(text) => {
                state.write_u8(7);
                text.hash(state);
            }

            Self::Blob(blob) => {
                state.write_u8(8);
                blob.hash(state);
            }

            Self::List(list) => {
                state.write_u8(9);
                list.hash(state);
            }

            Self::Map(map) => {
                state.write_u8(10);
                map.hash(state);
            }

            Self::Custom(kind, properties) => {
                state.write_u8(11);
                kind.hash(state);
                properties.hash(state);
            }

            Self::Call(call) => {
                state.write_u8(12);
                call.hash(state);
            }
        }
    }
}
