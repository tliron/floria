use super::{
    super::{data::*, store::*},
    class::*,
    property::*,
};

use {kutil::std::immutable::*, std::collections::*};

//
// Template
//

/// Template.
#[derive(Clone, Debug)]
pub struct Template {
    /// ID.
    pub id: ID,

    /// Metadata.
    pub metadata: Metadata,

    /// Class IDs.
    pub class_ids: Vec<ID>,

    /// Property templates.
    pub property_templates: BTreeMap<ByteString, Property>,

    /// Event handlers.
    pub event_handlers: BTreeMap<ByteString, FunctionName>,
}

impl Template {
    /// Constructor.
    pub fn new(id: ID) -> Self {
        Self {
            id,
            metadata: Default::default(),
            class_ids: Default::default(),
            property_templates: Default::default(),
            event_handlers: Default::default(),
        }
    }

    /// Into expression.
    pub fn into_expression<StoreT>(
        self,
        map: &mut BTreeMap<Expression, Expression>,
        embedded: bool,
        store: StoreT,
    ) -> Result<(), StoreError>
    where
        StoreT: Clone + Store,
    {
        map.insert("kind".into(), self.id.kind.as_str().into());
        map.insert("id".into(), self.id.to_string().into());
        map.insert("metadata".into(), metadata_into_expression(self.metadata));
        classes_into_expression(store.clone(), map, embedded, self.class_ids)?;
        properties_into_expression(store, map, "property-templates", embedded, self.property_templates)?;

        Ok(())
    }
}
