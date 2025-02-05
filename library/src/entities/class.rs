use super::{
    super::{data::*, store::*},
    utils::*,
};

use {
    kutil::{cli::depict::*, std::immutable::*},
    std::{collections::*, io},
};

//
// Class
//

/// Class.
#[derive(Clone, Debug)]
pub struct Class {
    /// ID.
    pub id: ID,

    /// Metadata.
    pub metadata: Metadata,

    /// TODO: Parent class IDs.
    pub parent_class_ids: Vec<ID>,

    /// TODO: Child class IDs.
    pub child_class_ids: Vec<ID>,
}

impl Class {
    /// Constructor.
    pub fn new_for(directory: Directory, id: ByteString) -> Self {
        Self::new_with(ID::new_for(Kind::Class, directory, id))
    }

    /// Constructor.
    pub fn new_with(id: ID) -> Self {
        Self {
            id,
            metadata: Default::default(),
            parent_class_ids: Default::default(),
            child_class_ids: Default::default(),
        }
    }

    /// To [Depict].
    pub fn to_depict<'own, StoreT>(&'own self, store: &'own StoreT) -> DepictClass<'own, StoreT>
    where
        StoreT: Store,
    {
        DepictClass { class: self, store }
    }
}

//
// DepictClass
//

/// Depict class.
#[allow(unused)]
pub struct DepictClass<'own, StoreT>
where
    StoreT: Store,
{
    class: &'own Class,
    store: &'own StoreT,
}

impl<'own, StoreT> Depict for DepictClass<'own, StoreT>
where
    StoreT: Store,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.theme.write_heading(writer, "Class")?;
        depict_id("id", Some(&self.class.id), false, writer, context)?;
        depict_metadata(&self.class.metadata, true, writer, context)?;
        Ok(())
    }
}

impl Into<Expression> for Class {
    fn into(self) -> Expression {
        let mut map = BTreeMap::default();

        map.insert("kind".into(), self.id.kind.as_str().into());
        map.insert("id".into(), self.id.to_string().into());
        map.insert("metadata".into(), metadata_into_expression(self.metadata));

        map.into()
    }
}
