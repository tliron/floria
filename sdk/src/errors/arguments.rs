/// Exact arguments.
pub fn arguments_exact(required: usize, actual: usize) -> String {
    format!("must have {}: |error|{}|", argument(required), actual)
}

/// Minimum arguments.
pub fn arguments_min(min: usize, actual: usize) -> String {
    format!("must have at least {}: |error|{}|", argument(min), actual)
}

/// Minimum arguments.
pub fn arguments_range(min: usize, max: usize, actual: usize) -> String {
    if min == 0 {
        format!("must have at most {}: |error|{}|", argument(min), actual)
    } else {
        format!("must have at between {} and {}: |error|{}|", count_to_string(min), argument(max), actual)
    }
}

fn argument(count: usize) -> String {
    let string = count_to_string(count);
    if count == 1 { format!("{} argument", string) } else { format!("{} arguments", string) }
}

fn count_to_string(count: usize) -> String {
    match count {
        0 => "no".to_string(),
        _ => count.to_string(),
    }
}
