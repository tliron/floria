use super::super::super::super::dispatch_bindings::*;

use std::{cmp::*, hash::*, mem::*};

impl PartialEq for CallKind {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for CallKind {}

impl PartialOrd for CallKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CallKind {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Normal, CallKind::Normal) => Ordering::Equal,
            (CallKind::Normal, _) => Ordering::Less,

            (CallKind::Eager, CallKind::Eager) => Ordering::Equal,
            (CallKind::Eager, CallKind::Normal) => Ordering::Greater,
            (CallKind::Eager, CallKind::Lazy) => Ordering::Less,

            (CallKind::Lazy, CallKind::Lazy) => Ordering::Equal,
            (CallKind::Lazy, _) => Ordering::Greater,
        }
    }
}

impl Hash for CallKind {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        match self {
            Self::Normal => state.write_u8(1),
            Self::Eager => state.write_u8(2),
            Self::Lazy => state.write_u8(3),
        }
    }
}
