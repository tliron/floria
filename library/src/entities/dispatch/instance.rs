use super::super::{
    super::{errors::*, plugins::*, store::*},
    instance::*,
};

use kutil::std::error::*;

impl Instance {
    /// Prepare properties. Returns true if any property was modified.
    pub fn prepare_properties<StoreT, ErrorReceiverT>(
        &mut self,
        library: &mut Library<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<bool, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        let mut modified = false;

        for (property_name, property) in self.properties.iter_mut() {
            if property.prepare(&self.id, &property_name, library, errors)? {
                modified = true;
            }
        }

        Ok(modified)
    }

    /// Update properties. Returns true if any property was modified.
    pub fn update_properties<StoreT, ErrorReceiverT>(
        &mut self,
        library: &mut Library<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<bool, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        let mut modified = false;

        for (property_name, property) in self.properties.iter_mut() {
            if property.update(&self.id, &property_name, library, errors)? {
                modified = true;
            }
        }

        Ok(modified)
    }
}
