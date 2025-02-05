use super::super::super::dispatch_bindings::*;

use std::{cmp::*, fmt, hash::*};

impl CustomResource {
    /// Get custom.
    pub fn custom(&self) -> &Custom {
        self.get()
    }

    /// Get custom.
    pub fn custom_mut(&mut self) -> &mut Custom {
        self.get_mut()
    }
}

//
// Custom
//

/// Custom.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Custom {
    /// Kind.
    pub kind: String,

    /// Inner.
    pub inner: Expression,
}

impl Custom {
    /// Get custom.
    pub fn new(kind: String, inner: Expression) -> Self {
        Self { kind, inner }
    }
}

impl GuestCustomResource for Custom {
    fn new(kind: String, inner: Expression) -> Self {
        Self { kind, inner }
    }

    fn get(&self) -> (String, Expression) {
        (self.kind.clone(), self.inner.clone())
    }
}

impl fmt::Display for Custom {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "kind: {}, inner: {}", self.kind, self.inner)
    }
}
