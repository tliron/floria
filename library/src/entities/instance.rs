use super::{
    super::{data::*, store::*},
    class::*,
    events::*,
    property::*,
    template::*,
};

use std::collections::*;

//
// Instance
//

/// Instance.
#[derive(Clone, Debug)]
pub struct Instance {
    /// ID.
    pub id: ID,

    /// Origin template ID.
    pub origin_template_id: Option<ID>,

    /// Metadata.
    pub metadata: Metadata,

    /// Class IDs.
    pub class_ids: Vec<ID>,

    /// Properties.
    pub properties: Properties,

    /// Event handlers.
    pub event_handlers: EventHandlers,
}

impl Instance {
    /// Constructor.
    pub fn new(id: ID, origin_template_id: Option<ID>) -> Self {
        Self {
            id,
            origin_template_id,
            metadata: Default::default(),
            class_ids: Default::default(),
            properties: Default::default(),
            event_handlers: Default::default(),
        }
    }

    /// Constructor.
    pub fn new_from_template<StoreT>(
        template: &Template,
        kind: EntityKind,
        directory: &Directory,
        store: StoreT,
    ) -> Result<Self, StoreError>
    where
        StoreT: Store,
    {
        let id = ID::new(kind, directory.clone(), store)?;
        let mut instance = Self::new(id, Some(template.id.clone()));
        instance.metadata = template.metadata.clone();
        instance.class_ids = template.class_ids.clone();
        instance.properties = template.property_templates.clone();
        instance.event_handlers = template.event_handlers.clone();
        Ok(instance)
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

        if let Some(origin_template_id) = &self.origin_template_id {
            map.insert("origin-template-id".into(), origin_template_id.to_string().into());
        }

        map.insert("metadata".into(), metadata_into_expression(self.metadata));
        classes_into_expression(store.clone(), map, embedded, self.class_ids)?;
        properties_into_expression(store, map, "properties", embedded, self.properties)?;

        Ok(())
    }
}
