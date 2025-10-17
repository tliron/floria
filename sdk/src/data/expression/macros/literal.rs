/// Creates an [Expression](crate::data::Expression) from a bare primitive expression.
#[macro_export]
macro_rules! expression {
    ( $value:expr $(,)? ) => {
        $crate::data::Expression::from($value)
    };
}

/// Creates a [Expression::List](crate::data::Expression::List) from a sequence of bare primitive expressions.
#[macro_export]
macro_rules! expression_list {
    () => (
        $crate::data::Expression::List(
            $crate::data::ListResource::new(
                $crate::data::List::default()
            )
        )
    );

    ( $( $value:expr ),+ $(,)? ) => (
        $crate::data::Expression::List(
            $crate::data::ListResource::new(
                $crate::data::List::from(
                    [ $( $crate::expression!( $value ) ),+ ]
                )
            )
        )
    );
}

/// Creates a [Expression::Map](crate::data::Expression::Map) from a sequence of key-value tuples.
#[macro_export]
macro_rules! expression_map {
    () => (
        $crate::data::Expression::Map(
            $crate::data::MapResource::new(
                $crate::data::Map::default()
            )
        )
    );

    ( $( ( $key:expr, $value:expr ) ),+ $(,)? ) => (
        $crate::data::Expression::Map(
            $crate::data::MapResource::new(
                $crate::data::Map::from(
                    ::std::collections::BTreeMap::from(
                        [ $( ( $crate::expression!( $key ), $crate::expression!( $value ) ) ),+ ]
                    )
                )
            )
        )
    );
}

/// Creates a [Vec]<[Expression](crate::data::Expression)> from a sequence of bare primitive expressions.
#[macro_export]
macro_rules! expression_vec {
    ( $( $value:expr ),* $(,)? ) => (
        vec![ $( $crate::expression!( $value ) ),* ]
    );
}

#[allow(unused_imports)]
pub use {expression, expression_list, expression_map, expression_vec};
