use super::{
    super::{data::*, store::*},
    property::*,
    utils::*,
};

use {kutil::std::immutable::*, std::collections::*};

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
    pub properties: BTreeMap<ByteString, Property>,
}

impl Instance {
    /// Constructor.
    pub fn new_for(kind: Kind, directory: Directory, id: ByteString, origin_template_id: Option<ID>) -> Self {
        Self::new_with(ID::new_for(kind, directory, id), origin_template_id)
    }

    /// Constructor.
    pub fn new_with(id: ID, origin_template_id: Option<ID>) -> Self {
        Self {
            id,
            origin_template_id,
            metadata: Default::default(),
            class_ids: Default::default(),
            properties: Default::default(),
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

        if let Some(origin_template_id) = &self.origin_template_id {
            map.insert("origin_template_id".into(), origin_template_id.to_string().into());
        }

        map.insert("metadata".into(), metadata_into_expression(self.metadata));
        classes_into_expression(store, map, embedded, self.class_ids)?;
        properties_into_expression(store, map, "properties", embedded, self.properties)?;

        Ok(())
    }
}
