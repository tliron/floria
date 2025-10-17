use super::{
    super::{super::super::*, cast::*, operations::*},
    number::*,
};

use {
    duplicate::*,
    std::{mem::*, ops::*},
};

impl Number {
    /// True if 1.
    pub fn is_one(self) -> bool {
        match self {
            Self::Integer(integer) => integer == 1,
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer == 1,
            Self::Float(float) => float == 1.,
        }
    }

    #[duplicate_item(
      symbol op    op_i64    op_u64;
      ["+"]  [add] [add_i64] [add_u64];
      ["-"]  [sub] [sub_i64] [sub_u64];
      ["*"]  [mul] [mul_i64] [mul_u64];
      ["/"]  [div] [div_i64] [div_u64];
    )]
    /// Math operation.
    ///
    /// If same is true will return an error if the types are not the same.
    pub fn op(self, right: Self, same: bool) -> Result<Self, DispatchError> {
        if same && (discriminant(&self) != discriminant(&right)) {
            return Err(same_type(self, right, symbol));
        }

        Ok(match (self, right) {
            (Self::Integer(left), Self::Integer(right)) => op_i64(left, right)?.into(),
            (Self::Integer(left), Self::UnsignedInteger(right)) => op_i64(left, into_i64(right)?)?.into(),
            (Self::Integer(left), Self::Float(right)) => into_f64(left)?.op(right).into(),
            (Self::UnsignedInteger(left), Self::UnsignedInteger(right)) => op_u64(left, right)?.into(),
            (Self::UnsignedInteger(left), Self::Integer(right)) => op_i64(into_i64(left)?, into_i64(right)?)?.into(),
            (Self::UnsignedInteger(left), Self::Float(right)) => into_f64(left)?.op(right).into(),
            (Self::Float(left), Self::Float(right)) => left.op(right).into(),
            (Self::Float(left), Self::Integer(right)) => left.op(into_f64(right)?).into(),
            (Self::Float(left), Self::UnsignedInteger(right)) => left.op(into_f64(right)?).into(),
        })
    }
}

fn same_type(left: Number, right: Number, delimiter: &str) -> DispatchError {
    format!("arguments not the same type: |error|{}| {} |error|{}|", left.type_name(), delimiter, right.type_name())
}
