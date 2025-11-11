use super::super::super::super::{super::store::*, bindings::floria::plugins::floria as bindings, errors::*, host::*};

impl<StoreT> PluginHost<StoreT>
where
    StoreT: Store,
{
    /// Whether two expressions are equal.
    pub fn expression_equal(
        &self,
        left: &bindings::Expression,
        right: &bindings::Expression,
    ) -> Result<bool, PluginError> {
        Ok(match (left, right) {
            (bindings::Expression::Null, bindings::Expression::Null) => true,
            (bindings::Expression::Integer(left), bindings::Expression::Integer(right)) => *left == *right,
            (bindings::Expression::UnsignedInteger(left), bindings::Expression::UnsignedInteger(right)) => {
                *left == *right
            }
            (bindings::Expression::Float(left), bindings::Expression::Float(right)) => *left == *right,
            (bindings::Expression::Boolean(left), bindings::Expression::Boolean(right)) => *left == *right,
            (bindings::Expression::Text(left), bindings::Expression::Text(right)) => *left == *right,
            (bindings::Expression::Blob(left), bindings::Expression::Blob(right)) => *left == *right,

            (bindings::Expression::List(left), bindings::Expression::List(right)) => {
                let left = &self.resources.get(left)?.inner;
                let right = &self.resources.get(right)?.inner;
                return self.expressions_equal(left, right);
            }

            (bindings::Expression::Map(left), bindings::Expression::Map(right)) => {
                let left = &self.resources.get(left)?.inner;
                let right = &self.resources.get(right)?.inner;
                return self.expression_pairs_equal(left, right);
            }

            (bindings::Expression::Custom(left), bindings::Expression::Custom(right)) => {
                let left = self.resources.get(left)?;
                let right = self.resources.get(right)?;
                if left.kind == right.kind {
                    return self.expression_equal(&left.inner, &right.inner);
                } else {
                    false
                }
            }

            (bindings::Expression::Call(left), bindings::Expression::Call(right)) => {
                let left = self.resources.get(left)?;
                let right = self.resources.get(right)?;
                if (left.kind == right.kind) && (left.plugin == right.plugin) && (left.function == right.function) {
                    return self.expressions_equal(&left.arguments, &right.arguments);
                } else {
                    false
                }
            }

            _ => false,
        })
    }

    /// Whether two expression lists are equal.
    pub fn expressions_equal(
        &self,
        left: &Vec<bindings::Expression>,
        right: &Vec<bindings::Expression>,
    ) -> Result<bool, PluginError> {
        if left.len() == right.len() {
            for (index, left) in left.iter().enumerate() {
                if !self.expression_equal(left, &right[index])? {
                    return Ok(false);
                }
            }
        }
        Ok(false)
    }

    /// Whether two expression pair lists are equal.
    pub fn expression_pairs_equal(
        &self,
        left: &Vec<(bindings::Expression, bindings::Expression)>,
        right: &Vec<(bindings::Expression, bindings::Expression)>,
    ) -> Result<bool, PluginError> {
        if left.len() == right.len() {
            for (left_key, left_value) in left {
                for (right_key, right_value) in right {
                    if self.expression_equal(left_key, right_key)? && !self.expression_equal(left_value, right_value)? {
                        return Ok(false);
                    }
                }
            }
        }
        Ok(false)
    }
}
