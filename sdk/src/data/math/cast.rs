use super::super::super::{errors::*, *};

use {num_traits::*, std::fmt};

/// Cast into integer.
pub fn into_i64<NumCastT>(number: NumCastT) -> Result<i64, DispatchError>
where
    NumCastT: Copy + fmt::Display + NumCast,
{
    cast(number).ok_or_else(|| integer_fit(number))
}

/// Cast into integer.
pub fn into_i32<NumCastT>(number: NumCastT) -> Result<i32, DispatchError>
where
    NumCastT: Copy + fmt::Display + NumCast,
{
    cast(number).ok_or_else(|| integer_fit(number))
}

/// Cast into unsigned integer.
pub fn into_u64<NumCastT>(number: NumCastT) -> Result<u64, DispatchError>
where
    NumCastT: Copy + fmt::Display + NumCast,
{
    cast(number).ok_or_else(|| unsigned_integer_fit(number))
}

/// Cast into unsigned integer.
pub fn into_u32<NumCastT>(number: NumCastT) -> Result<u32, DispatchError>
where
    NumCastT: Copy + fmt::Display + NumCast,
{
    cast(number).ok_or_else(|| unsigned_integer_fit(number))
}

/// Cast into float.
pub fn into_f64<NumCastT>(number: NumCastT) -> Result<f64, DispatchError>
where
    NumCastT: Copy + fmt::Display + NumCast,
{
    cast(number).ok_or_else(|| float_fit(number))
}
