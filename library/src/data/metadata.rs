use super::expression::*;

use compris::{annotate::*, normal::*};

/// Metadata.
pub type Metadata = Map<WithoutAnnotations>;

// Utils

/// Metadata into expression.
pub fn metadata_into_expression(metadata: Metadata) -> Expression {
    Expression::Map(metadata.into_iter().map(|(key, value)| (key.into(), value.into())).collect())
}
