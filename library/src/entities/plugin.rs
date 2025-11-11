use super::{
    super::{data::*, store::*},
    utils::*,
};

use {
    depiction::*,
    kutil::std::immutable::*,
    std::{collections::*, io},
};

//
// Plugin
//

/// Plugin.
#[derive(Clone, Debug)]
pub struct Plugin {
    /// ID.
    pub id: ID,

    /// URL.
    pub url: ByteString,
}

impl Plugin {
    /// Constructor.
    pub fn new_with(id: ID, url: ByteString) -> Self {
        Self { id, url }
    }

    /// To [Depict].
    pub fn to_depict<'own, StoreT>(&'own self, store: &'own StoreT) -> DepictPlugin<'own, StoreT>
    where
        StoreT: Store,
    {
        DepictPlugin { class: self, store }
    }
}

//
// DepictPlugin
//

/// Depict class.
#[allow(unused)]
pub struct DepictPlugin<'own, StoreT>
where
    StoreT: Store,
{
    class: &'own Plugin,
    store: &'own StoreT,
}

impl<'own, StoreT> Depict for DepictPlugin<'own, StoreT>
where
    StoreT: Store,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.theme.write_heading(writer, "Plugin")?;
        depict_id("id", Some(&self.class.id), false, writer, context)?;
        context.theme.write_string(writer, &self.class.url)
    }
}

impl Into<Expression> for Plugin {
    fn into(self) -> Expression {
        let mut map = BTreeMap::default();

        map.insert("kind".into(), self.id.kind.as_str().into());
        map.insert("id".into(), self.id.to_string().into());
        map.insert("path".into(), self.url.into());

        map.into()
    }
}
