use super::expression::*;

use {
    kutil::cli::depict::*,
    std::{cmp::*, fmt, hash::*, io},
};

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
                    context.theme.write_number(writer, integer)
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
                    context.theme.write_number(writer, float)
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
                context.theme.write_symbol(writer, kind)?;
                inner.depict(writer, context)
            }

            Self::Call(call) => call.depict(writer, context),
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
            (Self::Integer(integer), Self::Integer(other_integer)) => integer == other_integer,
            (Self::UnsignedInteger(unsigned_integer), Self::UnsignedInteger(other_unsigned_integer)) => {
                unsigned_integer == other_unsigned_integer
            }
            (Self::Float(float), Self::Float(other_float)) => float == other_float,
            (Self::Boolean(boolean), Self::Boolean(other_boolean)) => boolean == other_boolean,
            (Self::Text(text), Self::Text(other_text)) => text == other_text,
            (Self::Blob(blob), Self::Blob(other_blob)) => blob == other_blob,
            (Self::List(list), Self::List(other_list)) => list == other_list,
            (Self::Map(map), Self::Map(other_map)) => map == other_map,
            (Self::Custom(kind, properties), Self::Custom(other_kind, other_properties)) => {
                (kind == other_kind) && (properties == other_properties)
            }
            (Self::Call(call), Self::Call(other_call)) => call == other_call,

            _ => false,
        }
    }
}

impl Eq for Expression {}

impl PartialOrd for Expression {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Undefined, Self::Undefined) | (Self::Null, Self::Null) => Some(Ordering::Equal),
            (Self::Integer(integer), Self::Integer(other_integer)) => integer.partial_cmp(other_integer),
            (Self::UnsignedInteger(unsigned_integer), Self::UnsignedInteger(other_unsigned_integer)) => {
                unsigned_integer.partial_cmp(other_unsigned_integer)
            }
            (Self::Float(float), Self::Float(other_float)) => float.partial_cmp(other_float),
            (Self::Boolean(boolean), Self::Boolean(other_boolean)) => boolean.partial_cmp(other_boolean),
            (Self::Text(text), Self::Text(other_text)) => text.partial_cmp(other_text),
            (Self::Blob(blob), Self::Blob(other_blob)) => blob.partial_cmp(other_blob),
            (Self::List(list), Self::List(other_list)) => list.partial_cmp(other_list),
            (Self::Map(map), Self::Map(other_map)) => map.partial_cmp(other_map),
            (Self::Custom(kind, properties), Self::Custom(other_kind, other_properties)) => {
                match kind.partial_cmp(other_kind) {
                    Some(Ordering::Equal) => properties.partial_cmp(other_properties),
                    ordering => ordering,
                }
            }
            (Self::Call(call), Self::Call(other_call)) => call.partial_cmp(other_call),

            _ => None,
        }
    }
}

impl Ord for Expression {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Undefined, Self::Undefined) | (Self::Null, Self::Null) => Ordering::Equal,
            (Self::Integer(integer), Self::Integer(other_integer)) => integer.cmp(other_integer),
            (Self::UnsignedInteger(unsigned_integer), Self::UnsignedInteger(other_unsigned_integer)) => {
                unsigned_integer.cmp(other_unsigned_integer)
            }
            (Self::Float(float), Self::Float(other_float)) => float.cmp(other_float),
            (Self::Boolean(boolean), Self::Boolean(other_boolean)) => boolean.cmp(other_boolean),
            (Self::Text(text), Self::Text(other_text)) => text.cmp(other_text),
            (Self::Blob(blob), Self::Blob(other_blob)) => blob.cmp(other_blob),
            (Self::List(list), Self::List(other_list)) => list.cmp(other_list),
            (Self::Map(map), Self::Map(other_map)) => map.cmp(other_map),
            (Self::Custom(kind, properties), Self::Custom(other_kind, other_properties)) => {
                match kind.cmp(other_kind) {
                    Ordering::Equal => properties.cmp(other_properties),
                    ordering => ordering,
                }
            }
            (Self::Call(call), Self::Call(other_call)) => call.cmp(other_call),

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
