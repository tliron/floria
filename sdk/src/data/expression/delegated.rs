use super::super::super::dispatch_bindings::*;

use {
    ordered_float::*,
    std::{cmp::*, fmt, hash::*},
};

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Integer(left), Self::Integer(right)) => left == right,
            (Self::UnsignedInteger(left), Self::UnsignedInteger(right)) => left == right,
            (Self::Float(left), Self::Float(right)) => left == right,
            (Self::Boolean(left), Self::Boolean(right)) => left == right,
            (Self::Text(left), Self::Text(right)) => left == right,
            (Self::Blob(left), Self::Blob(right)) => left == right,
            (Self::List(left), Self::List(right)) => left.list() == right.list(),
            (Self::Map(left), Self::Map(right)) => left.map() == right.map(),
            (Self::Custom(left), Self::Custom(right)) => left.custom() == right.custom(),
            (Self::Call(left), Self::Call(right)) => left.call() == right.call(),
            _ => false,
        }
    }
}

impl Eq for Expression {}

impl PartialOrd for Expression {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Null, Self::Null) => Some(Ordering::Equal),
            (Self::Integer(left), Self::Integer(right)) => left.partial_cmp(right),
            (Self::UnsignedInteger(left), Self::UnsignedInteger(right)) => left.partial_cmp(right),
            (Self::Float(left), Self::Float(right)) => left.partial_cmp(right),
            (Self::Boolean(left), Self::Boolean(right)) => left.partial_cmp(right),
            (Self::Text(left), Self::Text(right)) => left.partial_cmp(right),
            (Self::Blob(left), Self::Blob(right)) => left.partial_cmp(right),
            (Self::List(left), Self::List(right)) => left.list().partial_cmp(right.list()),
            (Self::Map(left), Self::Map(right)) => left.map().partial_cmp(right.map()),
            (Self::Custom(left), Self::Custom(right)) => left.custom().partial_cmp(right.custom()),
            (Self::Call(left), Self::Call(right)) => left.call().partial_cmp(right.call()),
            _ => None,
        }
    }
}

impl Ord for Expression {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Null, Self::Null) => Ordering::Equal,
            (Self::Integer(left), Self::Integer(right)) => left.cmp(right),
            (Self::UnsignedInteger(left), Self::UnsignedInteger(right)) => left.cmp(right),
            (Self::Float(left), Self::Float(right)) => OrderedFloat::from(*left).cmp(&OrderedFloat::from(*right)),
            (Self::Boolean(left), Self::Boolean(right)) => left.cmp(right),
            (Self::Text(left), Self::Text(right)) => left.cmp(right),
            (Self::Blob(left), Self::Blob(right)) => left.cmp(right),
            (Self::List(left), Self::List(right)) => left.list().cmp(right.list()),
            (Self::Map(left), Self::Map(right)) => left.map().cmp(right.map()),
            (Self::Custom(left), Self::Custom(right)) => left.custom().cmp(right.custom()),
            (Self::Call(left), Self::Call(right)) => left.call().cmp(right.call()),

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
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Null => fmt::Display::fmt("Null", formatter),
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
