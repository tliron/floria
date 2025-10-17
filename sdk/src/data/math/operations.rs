use super::super::super::{errors::*, *};

/// Add integers.
pub fn add_i64(left: i64, right: i64) -> Result<i64, DispatchError> {
    left.checked_add(right).ok_or_else(|| integer_overflow(left, right, "+"))
}

/// Add unsigned integers.
pub fn add_u64(left: u64, right: u64) -> Result<u64, DispatchError> {
    left.checked_add(right).ok_or_else(|| unsigned_integer_overflow(left, right, "+"))
}

/// Subtract integers.
pub fn sub_i64(left: i64, right: i64) -> Result<i64, DispatchError> {
    left.checked_sub(right).ok_or_else(|| integer_overflow(left, right, "+"))
}

/// Subtract unsigned integers.
pub fn sub_u64(left: u64, right: u64) -> Result<u64, DispatchError> {
    left.checked_sub(right).ok_or_else(|| unsigned_integer_overflow(left, right, "+"))
}

/// Multiply integers.
pub fn mul_i64(left: i64, right: i64) -> Result<i64, DispatchError> {
    left.checked_mul(right).ok_or_else(|| integer_overflow(left, right, "*"))
}

/// Multiply unsigned integers.
pub fn mul_u64(left: u64, right: u64) -> Result<u64, DispatchError> {
    left.checked_mul(right).ok_or_else(|| unsigned_integer_overflow(left, right, "*"))
}

/// Divide integers.
pub fn div_i64(left: i64, right: i64) -> Result<i64, DispatchError> {
    left.checked_div(right).ok_or_else(|| integer_overflow(left, right, "/"))
}

/// Divide unsigned integers.
pub fn div_u64(left: u64, right: u64) -> Result<u64, DispatchError> {
    left.checked_div(right).ok_or_else(|| unsigned_integer_overflow(left, right, "/"))
}

/// Remainder for integers.
pub fn rem_i64(left: i64, right: i64) -> Result<i64, DispatchError> {
    left.checked_rem(right).ok_or_else(|| integer_overflow(left, right, "%"))
}
