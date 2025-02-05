use super::super::{
    super::{errors::*, plugins::*, store::*},
    instance::*,
};

use kutil::std::error::*;

impl Instance {
    /// Prepare.
    pub fn prepare<StoreT, ErrorRecipientT>(
        &mut self,
        library: &mut Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<bool, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorRecipientT: ErrorRecipient<FloriaError>,
    {
        let mut updated = false;

        for (property_name, property) in self.properties.iter_mut() {
            if property.prepare(&self.id, &property_name, library, errors)? {
                updated = true;
            }
        }

        Ok(updated)
    }

    /// Update.
    pub fn update<StoreT, ErrorRecipientT>(
        &mut self,
        library: &mut Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<bool, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorRecipientT: ErrorRecipient<FloriaError>,
    {
        let mut updated = false;

        for (property_name, property) in self.properties.iter_mut() {
            if property.update(&self.id, &property_name, library, errors)? {
                updated = true;
            }
        }

        Ok(updated)
    }
}
