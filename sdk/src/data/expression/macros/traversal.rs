// TODO!

/// Traverse a value by calling [Expression::get](crate::data::Expression::get) recursively.
///
/// The first argument is the starting [any::Any](crate::data::Expression). The following arguments
/// are a sequence of keys, which will be tried one at a time. Any non-collection or
/// missing key will cause the macro to stop and return [None].
///
/// The keys are either [Expression](crate::data::Expression) or anything that implements
/// [Into]<[Expression](crate::data::Expression)>, which includes all the supported primitive types.
#[macro_export]
macro_rules! traverse {
    ( $value:expr $(,)? ) => ( ::std::option::Option::<&$crate::functions::Expression>::Some(&$value) );

    ( $value:expr, $key:expr $(,)? ) => ( $value.into_get($key) );

    ( $value:expr, $key:expr, $( $next_key:expr ),+ $(,)? ) => (
        match $crate::traverse!( $value, $key ) {
            ::std::option::Option::Some(value) => $crate::traverse!( value $( , $next_key )+ ),
            ::std::option::Option::None => ::std::option::Option::None,
        }
    );
}

#[allow(unused_imports)]
pub use traverse;
