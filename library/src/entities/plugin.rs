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
// Plugin
//

/// Plugin.
#[derive(Clone, Debug)]
pub struct Plugin {
    /// ID.
    pub id: ID,

    /// URL.
    pub url: ByteString,

    /// True if precompiled.
    pub precompiled: bool,
}

impl Plugin {
    /// Constructor.
    pub fn new(id: ID, url: ByteString, precompiled: bool) -> Self {
        Self { id, url, precompiled }
    }

    /// Constructor.
    pub fn new_with_name(
        directory: Directory,
        name: ByteString,
        url: ByteString,
        precompiled: bool,
    ) -> Result<Self, MalformedError> {
        let id = ID::new_with_name(EntityKind::Plugin, directory, name)?;
        Ok(Self::new(id, url, precompiled))
    }

    /// Constructor.
    pub fn new_create_id<StoreT>(
        directory: Directory,
        url: ByteString,
        precompiled: bool,
        store: StoreT,
    ) -> Result<Self, Problem>
    where
        StoreT: Store,
    {
        let id = ID::new(EntityKind::Plugin, directory, store)?;
        Ok(Self::new(id, url, precompiled))
    }
}

impl Depict for Plugin {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.theme.write_heading(writer, "Plugin")?;

        depict_id("id", Some(&self.id), false, writer, context)?;

        depict_field("url", false, writer, context, |writer, context| {
            context.separate(writer)?;
            context.theme.write_string(writer, &self.url)
        })?;

        depict_field("precompiled", false, writer, context, |writer, context| {
            context.separate(writer)?;
            context.theme.write_symbol(writer, self.precompiled)
        })
    }
}

impl Into<Expression> for Plugin {
    fn into(self) -> Expression {
        let mut map = BTreeMap::default();

        map.insert("kind".into(), self.id.kind.as_str().into());
        map.insert("id".into(), self.id.to_string().into());
        map.insert("url".into(), self.url.into());
        map.insert("precompiled".into(), self.precompiled.into());

        map.into()
    }
}
