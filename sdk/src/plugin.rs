use super::{data::*, utils::*};

use std::sync::*;

/// Dispatch error.
///
/// Can contain depiction markup.
pub type DispatchError = String;

/// Dispatch result.
pub type DispatchResult = Result<Option<Expression>, DispatchError>;

type Static<StaticT> = LazyLock<Mutex<Option<StaticT>>>;

type Dispatch = fn(name: String, arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult;

static DISPATCH_PLUGIN: Static<(String, Dispatch)> = Static::new(|| Default::default());

/// Register dispatch plugin ID.
pub fn register_dispatch_plugin(id: String, dispatcher: Dispatch) -> Result<(), DispatchError> {
    *DISPATCH_PLUGIN.lock().map_escape_depiction_error()? = Some((id, dispatcher));
    Ok(())
}

/// Get registered dispatch plugin ID.
pub fn registered_dispatch_plugin() -> Result<(String, Dispatch), DispatchError> {
    match DISPATCH_PLUGIN.lock().map_escape_depiction_error()?.clone() {
        Some(id) => Ok(id),
        None => Err("dispatch plugin not registered".into()),
    }
}

/// Implement dispatcher plugin.
#[macro_export]
macro_rules! impl_dispatch {
    ( $arguments:ident, $call_site:ident, { $( $match:tt )* } ) => {
        /// Dispatch.
        pub struct Dispatch;

        $crate::export_dispatch!(Dispatch);

        impl $crate::dispatch_bindings::Guest for Dispatch {
            type ListResource = $crate::data::List;
            type MapResource = $crate::data::Map;
            type CustomResource = $crate::data::Custom;
            type CallResource = $crate::data::Call;

            fn initialize(id: String) -> ::std::result::Result<(), $crate::DispatchError> {
                $crate::register_dispatch_plugin(id, Self::dispatch)
            }

            fn dispatch(
                name: ::std::string::String,
                $arguments: ::std::vec::Vec<$crate::data::Expression>,
                $call_site: $crate::data::CallSite
            ) -> $crate::DispatchResult {
                match name.as_str() {
                    $( $match )*
                    _ => ::std::result::Result::Err("unsupported function".into()),
                }
            }
        }
    }
}

#[allow(unused_imports)]
pub use impl_dispatch;
