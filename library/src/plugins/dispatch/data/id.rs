use super::super::super::bindings::exports::floria::plugins::dispatch::*;

use std::cmp::*;

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        (self.kind == other.kind) && (self.directory == other.directory) && (self.name == other.name)
    }
}

impl Eq for Id {}

impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Id {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => {}
            ordering => return ordering,
        }

        match self.directory.cmp(&other.directory) {
            Ordering::Equal => {}
            ordering => return ordering,
        }

        self.name.cmp(&other.name)
    }
}
