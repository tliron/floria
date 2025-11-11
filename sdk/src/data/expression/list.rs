use super::super::super::dispatch_bindings::*;

use std::{
    cmp::*,
    fmt::{self, Write},
    hash::*,
};

impl ListResource {
    /// Into list.
    pub fn into_list(self) -> List {
        self.into_inner()
    }

    /// Get list.
    pub fn list(&self) -> &List {
        self.get()
    }

    /// Get list.
    pub fn list_mut(&mut self) -> &mut List {
        self.get_mut()
    }
}

//
// List
//

/// List.
#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct List {
    /// Inner.
    pub inner: Vec<Expression>,
}

impl GuestListResource for List {
    fn new(list: Vec<Expression>) -> Self {
        Self::from(list)
    }

    fn replica(&self) -> Vec<Expression> {
        self.inner.clone()
    }

    fn length(&self) -> u64 {
        self.inner.len() as u64
    }
}

impl From<Vec<Expression>> for List {
    fn from(inner: Vec<Expression>) -> Self {
        Self { inner }
    }
}

impl FromIterator<Expression> for List {
    fn from_iter<IntoIteratorT>(iter: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = Expression>,
    {
        Self::new(Vec::from_iter(iter))
    }
}

impl fmt::Display for List {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_char('[')?;

        let mut iterator = self.inner.iter().peekable();
        while let Some(item) = iterator.next() {
            fmt::Display::fmt(item, formatter)?;
            if iterator.peek().is_some() {
                formatter.write_char(',')?;
            }
        }

        formatter.write_char(']')
    }
}
