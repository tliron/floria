use super::{
    super::{data::*, store::*},
    class::*,
    event_handler::*,
    instance::*,
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
    pub event_handlers: Vec<EventHandler>,
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
    pub fn into_expression<'own, StoreT>(
        self,
        map: &mut BTreeMap<Expression, Expression>,
        embedded: bool,
        store: &'own StoreT,
    ) -> Result<(), StoreError>
    where
        StoreT: Store,
    {
        map.insert("kind".into(), self.id.kind.as_str().into());
        map.insert("id".into(), self.id.to_string().into());
        map.insert("metadata".into(), metadata_into_expression(self.metadata));
        classes_into_expression(store, map, embedded, self.class_ids)?;
        properties_into_expression(store, map, "property_templates", embedded, self.property_templates)?;

        Ok(())
    }

    /// Instantiate.
    pub fn instantiate<StoreT>(
        &self,
        kind: EntityKind,
        directory: &Directory,
        store: &StoreT,
    ) -> Result<Instance, StoreError>
    where
        StoreT: Store,
    {
        let mut id = ID::new(kind, directory.clone());
        store.create_id(&mut id)?;

        let mut instance = Instance::new_with(id, Some(self.id.clone()));
        instance.metadata = self.metadata.clone();
        instance.class_ids = self.class_ids.clone();
        instance.properties = self.property_templates.clone();

        Ok(instance)
    }
}
