use super::super::{data::*, floria_bindings};

//
// Metadata
//

/// Metadata.
pub trait Metadata {
    /// Metadata.
    fn metadata(&self) -> Vec<(floria_bindings::Expression, floria_bindings::Expression)>;

    /// Class IDs.
    fn class_ids(&self) -> &Vec<floria_bindings::Id>;

    /// Metadata expression.
    fn metadata_expression(&self, key: &str) -> Option<Expression> {
        for (key_, value) in self.metadata() {
            if let floria_bindings::Expression::Text(key_) = key_
                && key_ == key
            {
                return Some(value.into());
            }
        }
        None
    }

    /// Metadata string.
    fn metadata_string(&self, key: &str) -> Option<String> {
        for (key_, value) in self.metadata() {
            if let floria_bindings::Expression::Text(key_) = key_
                && key_ == key
            {
                return if let floria_bindings::Expression::Text(text) = value { Some(text) } else { None };
            }
        }
        None
    }

    /// Metadata map string.
    fn metadata_map_string(&self, map_key: &str, string_key: &str) -> Option<String> {
        for (key, value) in self.metadata() {
            if let floria_bindings::Expression::Text(key) = key
                && key == map_key
            {
                if let floria_bindings::Expression::Map(map_resource) = value {
                    for (key, value) in map_resource.inner() {
                        if let floria_bindings::Expression::Text(key_) = key
                            && key_ == string_key
                        {
                            return if let floria_bindings::Expression::Text(text) = value { Some(text) } else { None };
                        }
                    }
                }
            }
        }
        None
    }

    /// True if has class ID.
    fn has_class_id(&self, id: &Id) -> bool {
        self.class_ids().contains(&id.clone().into())
    }
}
