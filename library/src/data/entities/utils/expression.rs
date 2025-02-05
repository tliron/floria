use super::super::super::expression::*;

use std::{collections::*, fmt};

/// Displays into expressions.
pub fn displays_into_expressions<DisplayT>(
    map: &mut BTreeMap<Expression, Expression>,
    key: &'static str,
    displays: Vec<DisplayT>,
) where
    DisplayT: fmt::Display,
{
    let expressions: Vec<_> = displays.into_iter().map(|display| display.to_string().into()).collect();
    map.insert(key.into(), expressions.into());
}
