use super::{data::*, utils::*};

use std::sync::*;

/// Dispatch error.
///
/// Can contain depiction markup.
pub type DispatchError = String;

/// Dispatch result.
pub type DispatchResult = Result<Option<Expression>, DispatchError>;

type Static<T> = LazyLock<Mutex<Option<T>>>;

type Dispatcher = fn(name: String, arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult;

static DISPATCHER_PLUGIN: Static<(String, Dispatcher)> = Static::new(|| Default::default());

/// Register dispatcher plugin.
pub fn register_dispatcher_plugin(name: String, dispatcher: Dispatcher) -> Result<(), DispatchError> {
    *DISPATCHER_PLUGIN.lock().map_escape_depiction_error()? = Some((name, dispatcher));
    Ok(())
}

/// Get registered dispatcher plugin.
pub fn registered_dispatcher_plugin() -> Result<(String, Dispatcher), DispatchError> {
    match DISPATCHER_PLUGIN.lock().map_escape_depiction_error()?.clone() {
        Some(plugin) => Ok(plugin),
        None => Err("plugin not registered".into()),
    }
}

/// Implement dispatcher plugin.
#[macro_export]
macro_rules! impl_dispatcher (
    ( $plugin_name:expr, $arguments:ident, $call_site:ident, { $( $match:tt )* } ) => {
        /// Dispatcher.
        pub struct Dispatcher;

        $crate::export_dispatcher!(Dispatcher);

        impl $crate::dispatch_bindings::Guest for Dispatcher {
            type ListResource = $crate::data::List;
            type MapResource = $crate::data::Map;
            type CustomResource = $crate::data::Custom;
            type CallResource = $crate::data::Call;

            fn initialize() -> ::std::result::Result<(), $crate::DispatchError> {
                $crate::register_dispatcher_plugin($plugin_name.into(), Self::dispatch)
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
);

#[allow(unused_imports)]
pub use impl_dispatcher;
