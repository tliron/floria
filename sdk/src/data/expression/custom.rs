use super::super::super::{dispatch_bindings::*, utils::*, *};

use std::{cmp::*, fmt, hash::*};

impl CustomResource {
    /// Into custom.
    pub fn into_custom(self) -> Custom {
        self.into_inner()
    }

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
    /// Constructor.
    pub fn new(kind: String, inner: Expression) -> Self {
        Self { kind, inner }
    }

    /// Assert kind.
    pub fn assert_kind(&self, kind: &str, type_name: &str) -> Result<(), DispatchError> {
        if self.kind == kind {
            Ok(())
        } else {
            Err(format!(
                "{} custom kind not |name|{}|: |error|{}|",
                escape_depiction_markup(type_name),
                escape_depiction_markup(kind),
                escape_depiction_markup(&self.kind)
            ))
        }
    }
}

impl GuestCustomResource for Custom {
    fn new(kind: String, inner: Expression) -> Self {
        Self { kind, inner }
    }

    fn inner(&self) -> (String, Expression) {
        (self.kind.clone(), self.inner.clone())
    }
}

impl fmt::Display for Custom {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "kind: {}, inner: {}", self.kind, self.inner)
    }
}
