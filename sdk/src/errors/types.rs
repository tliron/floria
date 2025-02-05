use super::super::data::*;

/// Not of types.
pub fn not_of_types(expression: &Expression, types: &[&str]) -> String {
    let types: Vec<_> = types.into_iter().map(|t| format!("|name|{}|", t)).collect();
    format!("not {}: |error|{}|", types.join(", "), expression.type_name())
}

/// Not of types.
pub fn not_of_types_for(name: &str, expression: &Expression, types: &[&str]) -> String {
    format!("{} {}", name, not_of_types(expression, types))
}

/// Not the same type.
pub fn not_same_type(left: &Expression, right: &Expression, operator: &str) -> String {
    format!("not the same type: |error|{}| {} |error|{}|", left.type_name(), operator, right.type_name())
}
