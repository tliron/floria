use super::super::super::dispatch_bindings::*;

/// Convert vector of [Expression] to vector of string.
pub fn expression_vec_to_strings(values: &Vec<Expression>) -> Vec<String> {
    values.iter().map(|expression| expression.to_string()).collect()
}

/// Convert vector of [Expression] to string.
pub fn expression_vec_to_string(expression: &Vec<Expression>) -> String {
    expression_vec_to_strings(expression).join(",")
}
