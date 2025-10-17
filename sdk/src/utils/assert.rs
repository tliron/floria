use super::super::{data::*, *};

/// Assert argument count.
pub fn assert_argument_count(arguments: &Vec<Expression>, required: usize) -> Result<(), DispatchError> {
    let actual = arguments.len();
    if actual == required { Ok(()) } else { Err(errors::arguments_exact(required, actual)) }
}

/// Assert argument count.
pub fn assert_argument_count_min(arguments: &Vec<Expression>, min: usize) -> Result<(), DispatchError> {
    let actual = arguments.len();
    if actual >= min { Ok(()) } else { Err(errors::arguments_min(min, actual)) }
}

/// Assert argument count.
pub fn assert_argument_count_range(arguments: &Vec<Expression>, min: usize, max: usize) -> Result<(), DispatchError> {
    let actual = arguments.len();
    if (actual >= min) && (actual <= max) { Ok(()) } else { Err(errors::arguments_range(min, max, actual)) }
}
