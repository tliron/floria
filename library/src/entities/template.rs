use super::{
    super::{data::*, store::*},
    class::*,
    events::*,
    property::*,
};

use {problemo::*, std::collections::*};

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
    pub property_templates: Properties,

    /// Event handlers.
    ///
    /// TODO: return value to affect propagation?
    pub event_handlers: EventHandlers,

    /// Instantiation event handlers.
    ///
    /// "prepare": before
    ///
    /// "instantiate": after
    ///
    /// If any returns false then instantiation is cancelled.
    pub instantiation_event_handlers: EventHandlers,
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
            instantiation_event_handlers: Default::default(),
        }
    }

    /// Into expression.
    pub fn into_expression<StoreT>(
        self,
        map: &mut BTreeMap<Expression, Expression>,
        embedded: bool,
        store: StoreT,
    ) -> Result<(), Problem>
    where
        StoreT: Clone + Store,
    {
        map.insert("kind".into(), self.id.kind.as_str().into());
        map.insert("id".into(), self.id.to_string().into());
        map.insert("metadata".into(), metadata_into_expression(self.metadata));
        classes_into_expression(store.clone(), map, embedded, self.class_ids)?;
        properties_into_expression(store, map, "property-templates", embedded, self.property_templates)?;
        event_handlers_into_expression(map, self.event_handlers);

        Ok(())
    }
}
