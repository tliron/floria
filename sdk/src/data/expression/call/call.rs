use super::super::super::super::{dispatch_bindings::*, host, *};

use std::{cmp::*, fmt, hash::*};

impl CallResource {
    /// Into call.
    pub fn into_call(self) -> Call {
        self.into_inner()
    }

    /// Get call.
    pub fn call(&self) -> &Call {
        self.get()
    }

    /// Get call.
    pub fn call_mut(&mut self) -> &mut Call {
        self.get_mut()
    }
}

//
// Call
//

/// Call.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Call {
    /// Plugin name.
    pub plugin: String,

    /// Function name.
    pub function: String,

    /// Arguments.
    pub arguments: Vec<Expression>,

    /// Kind.
    pub kind: CallKind,
}

impl Call {
    /// Dispatch.
    pub fn dispatch(&self, call_site: &CallSite) -> DispatchResult {
        let (name, dispatcher) = registered_dispatch_plugin()?;
        if self.plugin == name {
            dispatcher(self.function.clone(), self.arguments.clone(), call_site.clone())
        } else {
            host::evaluate_expression(self.clone().into(), call_site.clone())
        }
    }
}

impl GuestCallResource for Call {
    fn new(plugin: String, function: String, arguments: Vec<Expression>, kind: CallKind) -> Self {
        Self { plugin, function, arguments, kind }
    }

    fn replica(&self) -> (String, String, Vec<Expression>, CallKind) {
        (self.plugin.clone(), self.function.clone(), self.arguments.clone(), self.kind)
    }
}

impl fmt::Display for Call {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            CallKind::Eager => write!(formatter, "*")?,
            CallKind::Lazy => write!(formatter, "&")?,
            _ => {}
        }

        write!(formatter, "{}:{}(", self.plugin, self.function)?;

        let mut iterator = self.arguments.iter().peekable();
        while let Some(argument) = iterator.next() {
            fmt::Display::fmt(argument, formatter)?;
            if iterator.peek().is_some() {
                write!(formatter, ",")?;
            }
        }

        write!(formatter, ")")
    }
}
