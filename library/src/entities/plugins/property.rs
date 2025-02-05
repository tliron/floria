use super::super::{
    super::{data::*, errors::*, plugins::*, store::*},
    property::*,
};

use kutil::std::error::*;

impl Property {
    /// Prepare.
    pub fn prepare<StoreT, ErrorRecipientT>(
        &mut self,
        id: &ID,
        property_name: &str,
        library: &mut Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<bool, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorRecipientT: ErrorRecipient<FloriaError>,
    {
        if let Some(preparer) = &self.preparer {
            let call_site = CallSite::new(id.clone(), vec![property_name.into()]);
            if let Some(value) = preparer.call(&call_site, library, errors)? {
                self.value = Some(value.into());
            }
            return Ok(true);
        }

        Ok(false)
    }

    /// Update.
    pub fn update<StoreT, ErrorRecipientT>(
        &mut self,
        id: &ID,
        property_name: &str,
        library: &mut Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<bool, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorRecipientT: ErrorRecipient<FloriaError>,
    {
        if self.read_only && self.value.is_some() {
            // Read-only properties can only be updated once.
            return Ok(false);
        }

        if let Some(updater) = &self.updater {
            let call_site = CallSite::new(id.clone(), vec![property_name.into()]);
            if let Some(value) = updater.call(&call_site, library, errors)? {
                self.value = Some(value.into());
            }
            return Ok(true);
        }

        Ok(false)
    }
}
