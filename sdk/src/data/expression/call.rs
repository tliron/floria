use super::super::super::dispatch_bindings::*;

use std::{cmp::*, fmt, hash::*};

impl CallResource {
    /// To call.
    pub fn call(&self) -> &Call {
        self.get()
    }

    /// To call.
    pub fn to_call_mut(&mut self) -> &mut Call {
        self.get_mut()
    }
}

//
// Call
//

/// Call.
#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Call {
    /// Plugin name.
    pub plugin: String,

    /// Function name.
    pub function: String,

    /// Arguments.
    pub arguments: Vec<Expression>,
}

impl GuestCallResource for Call {
    fn new(plugin: String, function: String, arguments: Vec<Expression>) -> Self {
        Self { plugin, function, arguments }
    }

    fn get(&self) -> (String, String, Vec<Expression>) {
        (self.plugin.clone(), self.function.clone(), self.arguments.clone())
    }
}

impl fmt::Display for Call {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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
