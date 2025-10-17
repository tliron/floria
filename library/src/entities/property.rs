use super::{
    super::{data::*, store::*},
    class::*,
    utils::*,
};

use {
    depiction::*,
    kutil::std::immutable::*,
    std::{collections::*, io},
};

//
// Property
//

/// Property.
///
/// Equivalent to TOSCA property or attribute.
#[derive(Clone, Debug)]
pub struct Property {
    /// Metadata.
    pub metadata: Metadata,

    /// Class IDs.
    pub class_ids: Vec<ID>,

    /// Read-only.
    pub read_only: bool,

    /// Preparer.
    pub preparer: Option<Expression>,

    /// Updater.
    pub updater: Option<Expression>,

    /// Value.
    pub value: Option<Expression>,
}

impl Property {
    /// Constructor.
    pub fn new(
        read_only: bool,
        preparer: Option<Expression>,
        updater: Option<Expression>,
        value: Option<Expression>,
    ) -> Self {
        Self { read_only, metadata: Default::default(), class_ids: Default::default(), preparer, updater, value }
    }

    /// Into expression.
    pub fn into_expression<'own, StoreT>(self, embedded: bool, store: &'own StoreT) -> Result<Expression, StoreError>
    where
        StoreT: Store,
    {
        let mut map = BTreeMap::default();

        map.insert("metadata".into(), metadata_into_expression(self.metadata));
        classes_into_expression(store, &mut map, embedded, self.class_ids)?;

        map.insert("read-only".into(), self.read_only.into());

        if let Some(value) = self.value {
            map.insert("value".into(), value);
        }

        if let Some(preparer) = self.preparer {
            map.insert("preparer".into(), preparer);
        }

        if let Some(updater) = self.updater {
            map.insert("updater".into(), updater);
        }

        Ok(Expression::Map(map))
    }

    /// To [Depict].
    pub fn to_depict<'own, StoreT>(&'own self, store: &'own StoreT) -> DepictProperty<'own, StoreT>
    where
        StoreT: Store,
    {
        DepictProperty { property: self, store }
    }
}

//
// DepictProperty
//

/// Depict property.
pub struct DepictProperty<'own, StoreT>
where
    StoreT: Store,
{
    property: &'own Property,
    store: &'own StoreT,
}

impl<'own, StoreT> Depict for DepictProperty<'own, StoreT>
where
    StoreT: Store,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let context = &context.child().with_separator(true);

        context.separate(writer)?;
        context.theme.write_heading(writer, "Property")?;
        depict_metadata(&self.property.metadata, false, writer, context)?;
        depict_classes(&self.property.class_ids, self.store, writer, context)?;

        utils::depict_field("read_only", false, writer, context, |writer, context| {
            context.separate(writer)?;
            context.theme.write_symbol(writer, self.property.read_only)
        })?;

        utils::depict_field("preparer", false, writer, context, |writer, context| match &self.property.preparer {
            Some(preparer) => preparer.depict(writer, context),
            None => {
                context.separate(writer)?;
                context.theme.write_symbol(writer, "None")
            }
        })?;

        utils::depict_field("updater", false, writer, context, |writer, context| match &self.property.updater {
            Some(updater) => updater.depict(writer, context),
            None => {
                context.separate(writer)?;
                context.theme.write_symbol(writer, "None")
            }
        })?;

        utils::depict_field("value", true, writer, context, |writer, context| match &self.property.value {
            Some(value) => value.depict(writer, context),
            None => {
                context.separate(writer)?;
                context.theme.write_symbol(writer, "None")
            }
        })
    }
}

// Utils

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
