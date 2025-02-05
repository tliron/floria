use super::super::{
    super::{data::*, store::*},
    property::*,
};

use {
    kutil::std::immutable::*,
    std::{collections::*, fmt},
};

/// Metadata into expression.
pub fn metadata_into_expression(metadata: Metadata) -> Expression {
    Expression::Map(metadata.into_iter().map(|(key, value)| (key.into(), value.into())).collect())
}

/// Classes into expression.
pub fn classes_into_expression<StoreT>(
    store: &StoreT,
    map: &mut BTreeMap<Expression, Expression>,
    embedded: bool,
    class_ids: Vec<ID>,
) -> Result<(), StoreError>
where
    StoreT: Store,
{
    if class_ids.is_empty() {
        return Ok(());
    }

    if embedded {
        let mut classes = Vec::with_capacity(class_ids.len());
        for class_id in class_ids {
            if let Some(class) = store.get_class(&class_id)? {
                classes.push(class.into());
            }
        }
        map.insert("classes".into(), classes.into());
    } else {
        let class_ids: Vec<_> = class_ids.into_iter().map(|id| id.to_string().into()).collect();
        map.insert("class_ids".into(), class_ids.into());
    }

    Ok(())
}

/// Properties into expression.
pub fn properties_into_expression<StoreT>(
    store: &StoreT,
    map: &mut BTreeMap<Expression, Expression>,
    key: &'static str,
    embedded: bool,
    properties: BTreeMap<ByteString, Property>,
) -> Result<(), StoreError>
where
    StoreT: Store,
{
    if properties.is_empty() {
        return Ok(());
    }

    let mut expressions = BTreeMap::default();
    for (property_name, property) in properties {
        expressions.insert(property_name.into(), property.into_expression(embedded, store)?);
    }

    map.insert(key.into(), expressions.into());

    Ok(())
}

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
