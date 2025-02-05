use super::super::super::{dispatch_bindings::*, floria_bindings};

//
// Metadata
//

/// Metadata.
pub trait Metadata {
    /// Metadata.
    fn metadata(&self) -> Vec<(floria_bindings::Expression, floria_bindings::Expression)>;

    /// Class IDs.
    fn class_ids(&self) -> &Vec<floria_bindings::Id>;

    /// Metadata value.
    fn metadata_value(&self, key: &str) -> Option<Expression> {
        for (key_, value) in self.metadata() {
            if let floria_bindings::Expression::Text(key_) = key_
                && key_ == key
            {
                return Some(value.into());
            }
        }
        None
    }

    /// Metadata string value.
    fn metadata_string(&self, key: &str) -> Option<String> {
        if let Some(Expression::Text(text)) = self.metadata_value(key) { Some(text) } else { None }
    }

    /// True if has class ID.
    fn has_class_id(&self, id: &Id) -> bool {
        self.class_ids().contains(&id.clone().into())
    }
}
