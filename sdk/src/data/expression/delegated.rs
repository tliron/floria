use super::super::super::dispatch_bindings::*;

use {
    ordered_float::*,
    std::{cmp::*, fmt, hash::*},
};

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Integer(integer), Self::Integer(other_integer)) => integer == other_integer,
            (Self::UnsignedInteger(unsigned_integer), Self::UnsignedInteger(other_unsigned_integer)) => {
                unsigned_integer == other_unsigned_integer
            }
            (Self::Float(float), Self::Float(other_float)) => float == other_float,
            (Self::Boolean(boolean), Self::Boolean(other_boolean)) => boolean == other_boolean,
            (Self::Text(text), Self::Text(other_text)) => text == other_text,
            (Self::Blob(blob), Self::Blob(other_blob)) => blob == other_blob,
            (Self::List(list_resource), Self::List(other_list_resource)) => {
                list_resource.list() == other_list_resource.list()
            }
            (Self::Map(map_resource), Self::Map(other_map_resource)) => map_resource.map() == other_map_resource.map(),
            (Self::Custom(custom_resource), Self::Custom(other_custom_resource)) => {
                custom_resource.custom() == other_custom_resource.custom()
            }
            (Self::Call(call_resource), Self::Call(other_call_resource)) => {
                call_resource.call() == other_call_resource.call()
            }

            _ => false,
        }
    }
}

impl Eq for Expression {}

impl PartialOrd for Expression {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Null, Self::Null) => Some(Ordering::Equal),
            (Self::Integer(integer), Self::Integer(other_integer)) => integer.partial_cmp(other_integer),
            (Self::UnsignedInteger(unsigned_integer), Self::UnsignedInteger(other_unsigned_integer)) => {
                unsigned_integer.partial_cmp(other_unsigned_integer)
            }
            (Self::Float(float), Self::Float(other_float)) => float.partial_cmp(other_float),
            (Self::Boolean(boolean), Self::Boolean(other_boolean)) => boolean.partial_cmp(other_boolean),
            (Self::Text(text), Self::Text(other_text)) => text.partial_cmp(other_text),
            (Self::Blob(blob), Self::Blob(other_blob)) => blob.partial_cmp(other_blob),
            (Self::List(list_resource), Self::List(other_list_resource)) => {
                list_resource.list().partial_cmp(other_list_resource.list())
            }
            (Self::Map(map_resource), Self::Map(other_map_resource)) => {
                map_resource.map().partial_cmp(other_map_resource.map())
            }
            (Self::Custom(custom_resource), Self::Custom(other_custom_resource)) => {
                custom_resource.custom().partial_cmp(other_custom_resource.custom())
            }
            (Self::Call(call_resource), Self::Call(other_call_resource)) => {
                call_resource.call().partial_cmp(other_call_resource.call())
            }

            _ => None,
        }
    }
}

impl Ord for Expression {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Null, Self::Null) => Ordering::Equal,
            (Self::Integer(integer), Self::Integer(other_integer)) => integer.cmp(other_integer),
            (Self::UnsignedInteger(unsigned_integer), Self::UnsignedInteger(other_unsigned_integer)) => {
                unsigned_integer.cmp(other_unsigned_integer)
            }
            (Self::Float(float), Self::Float(other_float)) => {
                OrderedFloat::from(*float).cmp(&OrderedFloat::from(*other_float))
            }
            (Self::Boolean(boolean), Self::Boolean(other_boolean)) => boolean.cmp(other_boolean),
            (Self::Text(text), Self::Text(other_text)) => text.cmp(other_text),
            (Self::Blob(blob), Self::Blob(other_blob)) => blob.cmp(other_blob),
            (Self::List(list_resource), Self::List(other_list_resource)) => {
                list_resource.list().cmp(other_list_resource.list())
            }
            (Self::Map(map_resource), Self::Map(other_map_resource)) => {
                map_resource.map().cmp(other_map_resource.map())
            }
            (Self::Custom(custom_resource), Self::Custom(other_custom_resource)) => {
                custom_resource.custom().cmp(other_custom_resource.custom())
            }
            (Self::Call(call_resource), Self::Call(other_call_resource)) => {
                call_resource.call().cmp(other_call_resource.call())
            }

            (Self::Null, _) => Ordering::Less,

            (Self::Integer(_), Self::Null) => Ordering::Greater,
            (Self::Integer(_), _) => Ordering::Less,

            (Self::UnsignedInteger(_), Self::Null | Self::Integer(_)) => Ordering::Greater,
            (Self::UnsignedInteger(_), _) => Ordering::Less,

            (Self::Float(_), Self::Null | Self::Integer(_) | Self::UnsignedInteger(_)) => Ordering::Greater,
            (Self::Float(_), _) => Ordering::Less,

            (Self::Boolean(_), Self::Null | Self::Integer(_) | Self::UnsignedInteger(_) | Self::Float(_)) => {
                Ordering::Greater
            }
            (Self::Boolean(_), _) => Ordering::Less,

            (
                Self::Text(_),
                Self::Null | Self::Integer(_) | Self::UnsignedInteger(_) | Self::Float(_) | Self::Boolean(_),
            ) => Ordering::Greater,
            (Self::Text(_), _) => Ordering::Less,

            (
                Self::Blob(_),
                Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_),
            ) => Ordering::Greater,
            (Self::Blob(_), _) => Ordering::Less,

            (
                Self::List(_),
                Self::Null
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
                Self::Null
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
                Self::Custom(_),
                Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_)
                | Self::Blob(_)
                | Self::List(_)
                | Self::Map(_),
            ) => Ordering::Greater,
            (Self::Custom(_), _) => Ordering::Less,

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
            Self::Null => {
                state.write_u8(1);
            }

            Self::Integer(integer) => {
                state.write_u8(2);
                integer.hash(state);
            }

            Self::UnsignedInteger(unsigned_integer) => {
                state.write_u8(3);
                unsigned_integer.hash(state);
            }

            Self::Float(float) => {
                state.write_u8(4);
                OrderedFloat::from(*float).hash(state);
            }

            Self::Boolean(boolean) => {
                state.write_u8(5);
                boolean.hash(state);
            }

            Self::Text(text) => {
                state.write_u8(6);
                text.hash(state);
            }

            Self::Blob(blob) => {
                state.write_u8(7);
                blob.hash(state);
            }

            Self::List(list_resource) => {
                state.write_u8(8);
                list_resource.list().hash(state);
            }

            Self::Map(map_resource) => {
                state.write_u8(9);
                map_resource.map().hash(state);
            }

            Self::Custom(custom_resource) => {
                state.write_u8(10);
                custom_resource.custom().hash(state);
            }

            Self::Call(call_resource) => {
                state.write_u8(11);
                call_resource.call().hash(state);
            }
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => fmt::Display::fmt("null", formatter),
            Self::Integer(integer) => fmt::Display::fmt(integer, formatter),
            Self::UnsignedInteger(unsigned_integer) => fmt::Display::fmt(unsigned_integer, formatter),
            Self::Float(float) => fmt::Display::fmt(float, formatter),
            Self::Boolean(boolean) => fmt::Display::fmt(boolean, formatter),
            Self::Text(text) => fmt::Debug::fmt(text, formatter),
            Self::Blob(blob) => write!(formatter, "{} bytes", blob.len()),
            Self::List(list_resource) => fmt::Display::fmt(list_resource.list(), formatter),
            Self::Map(map_resource) => fmt::Display::fmt(map_resource.map(), formatter),
            Self::Custom(custom_resource) => fmt::Display::fmt(custom_resource.custom(), formatter),
            Self::Call(call_resource) => fmt::Display::fmt(call_resource.call(), formatter),
        }
    }
}
