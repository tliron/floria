use super::super::{data::*, floria_bindings};

//
// Metadata
//

/// Metadata.
pub trait Metadata {
    /// Metadata.
    fn metadata(&self) -> &floria_bindings::MapResource;

    /// Class IDs.
    fn class_ids(&self) -> &Vec<floria_bindings::Id>;

    /// Get metadata.
    fn get_metadata(&self, key: &str) -> Option<Expression> {
        self.metadata().get(Expression::from(key).into()).map(|expression| expression.into())
    }

    /// Get metadata string.
    fn get_metadata_string(&self, key: &str) -> Option<String> {
        match self.get_metadata(key) {
            Some(Expression::Text(text)) => Some(text),
            _ => None,
        }
    }

    /// Get metadata string.
    fn get_metadata_sub_string(&self, key1: &str, key2: &str) -> Option<String> {
        match self.get_metadata(key1) {
            Some(Expression::Map(map)) => match map.map().into_get(key2) {
                Some(Expression::Text(text)) => Some(text.clone()),
                _ => None,
            },
            _ => None,
        }
    }

    /// True if has class ID.
    fn has_class_id(&self, id: &Id) -> bool {
        self.class_ids().contains(&id.clone().into())
    }
}
