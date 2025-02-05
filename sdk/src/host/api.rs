use super::super::{data, dispatch_bindings, floria_bindings};

// These wrappers expose dispatch types, internally converting to and from host types

/// Log.
#[macro_export]
macro_rules! log {
    ( $source:expr, $( $arg:tt )* ) => (
        $crate::floria_bindings::log($source, &::std::format! ( $( $arg )* ) )
    );
}

/// Evaluate expression.
pub fn evaluate_expression(
    expression: &dispatch_bindings::Expression,
    call_site: &dispatch_bindings::CallSite,
) -> Result<Option<data::Expression>, String> {
    Ok(floria_bindings::evaluate_expression(expression.clone().into(), &call_site.clone().into())?
        .map(|value| value.into()))
}

/// Get entity.
pub fn get_entity(id: &dispatch_bindings::Id) -> Result<data::Expression, String> {
    floria_bindings::get_entity(&id.clone().into()).map(|value| value.into())
}
