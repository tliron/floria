use super::{
    super::{data::*, store::*},
    utils::*,
};

use {
    depiction::*,
    kutil::std::immutable::*,
    problemo::{common::*, *},
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
}

impl Class {
    /// Constructor.
    pub fn new(id: ID) -> Self {
        Self { id, metadata: Default::default() }
    }

    /// Constructor.
    pub fn new_with_name(directory: Directory, name: ByteString) -> Result<Self, MalformedError> {
        let id = ID::new_with_name(EntityKind::Class, directory, name)?;
        Ok(Self::new(id))
    }

    /// As [Depict].
    pub fn as_depict<'this, 'store, 'depict, StoreT>(&'this self, store: &'store StoreT) -> DepictClass<'depict, StoreT>
    where
        'this: 'depict,
        'store: 'depict,
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
pub struct DepictClass<'inner, StoreT>
where
    StoreT: Store,
{
    class: &'inner Class,
    store: &'inner StoreT,
}

impl<'inner, StoreT> Depict for DepictClass<'inner, StoreT>
where
    StoreT: Store,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.theme.write_heading(writer, "Class")?;
        depict_id("id", Some(&self.class.id), false, writer, context)?;
        depict_metadata(&self.class.metadata, true, writer, context)
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

// Utils

/// Classes into expression.
pub fn classes_into_expression<StoreT>(
    store: StoreT,
    map: &mut BTreeMap<Expression, Expression>,
    embedded: bool,
    class_ids: Vec<ID>,
) -> Result<(), Problem>
where
    StoreT: Store,
{
    if class_ids.is_empty() {
        return Ok(());
    }

    if embedded {
        let mut classes = Vec::<Expression>::with_capacity(class_ids.len());
        for class_id in class_ids {
            if let Some(class) = store.get_class(&class_id)? {
                classes.push(class.into());
            }
        }
        map.insert("classes".into(), classes.into());
    } else {
        let class_ids: Vec<Expression> = class_ids.into_iter().map(|id| id.to_string().into()).collect();
        map.insert("class_ids".into(), class_ids.into());
    }

    Ok(())
}
