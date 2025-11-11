use super::super::*;

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
    expression: dispatch_bindings::Expression,
    call_site: dispatch_bindings::CallSite,
) -> Result<Option<data::Expression>, DispatchError> {
    Ok(floria_bindings::evaluate_expression(expression.into(), &call_site.into())?.map(|value| value.into()))
}

/// Get entity.
pub fn get_entity(id: &dispatch_bindings::Id) -> Result<floria_bindings::Entity, DispatchError> {
    floria_bindings::get_entity(&id.clone().into())
}

/// Add entity.
pub fn add_entity(entity: floria_bindings::Entity) -> Result<(), DispatchError> {
    floria_bindings::add_entity(entity)
}
