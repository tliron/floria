use kutil::std::error::ErrorRecipient;

use super::{
    super::super::{errors::*, plugins, store::*},
    expression::*,
};

use std::collections::*;

impl Expression {
    /// Call the expression if it is a call, otherwise [evaluate](Self::evaluate) it.
    pub fn call<StoreT, ErrorRecipientT>(
        &self,
        call_site: &plugins::CallSite,
        library: &mut plugins::Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<Expression>, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorRecipientT: ErrorRecipient<FloriaError>,
    {
        if let Self::Call(call) = self {
            call.call(call_site, library, errors)
        } else {
            self.evaluate(call_site, library, errors)
        }
    }

    /// Evaluate the expression.
    ///
    /// Eager calls will happen here.
    pub fn evaluate<StoreT, ErrorRecipientT>(
        &self,
        call_site: &plugins::CallSite,
        library: &mut plugins::Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<Expression>, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorRecipientT: ErrorRecipient<FloriaError>,
    {
        match self {
            Self::List(list) => {
                let mut expressions = Vec::with_capacity(list.len());
                for item in list {
                    expressions.push(item.evaluate(call_site, library, errors)?.unwrap_or_default());
                }
                Ok(Some(expressions.into()))
            }

            Self::Map(map) => {
                let mut expressions = BTreeMap::default();
                for (key, value) in map {
                    if let Some(key) = key.evaluate(call_site, library, errors)? {
                        expressions.insert(key, value.evaluate(call_site, library, errors)?.unwrap_or_default());
                    }
                }
                Ok(Some(expressions.into()))
            }

            Self::Call(call) => {
                if call.eager {
                    call.call(call_site, library, errors)
                } else {
                    Ok(Some(self.clone()))
                }
            }

            _ => Ok(Some(self.clone())),
        }
    }
}
