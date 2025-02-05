use super::{
    super::super::{dispatch_bindings::*, floria_bindings},
    metadata::*,
};

//
// Instance
//

/// Instance.
pub trait Instance: Metadata {
    /// ID.
    fn id(&self) -> Id;

    /// Properties.
    fn properties(&self) -> &Vec<(String, floria_bindings::Property)>;

    /// Property.
    fn property(&self, name: &str) -> Option<&floria_bindings::Property> {
        for (key, value) in self.properties() {
            if key == name {
                return Some(value);
            }
        }
        None
    }
}
