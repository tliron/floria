use super::{
    super::{data::*, store::*},
    malformed::*,
};

use {
    depiction::*,
    kutil::std::{collections::*, error::*, iter::*},
    std::io,
    thiserror::*,
};

//
// FloriaError
//

/// Floria error.
#[derive(Debug, Depict, Error)]
pub enum FloriaError {
    /// Instantiation.
    #[error("instantiation: {0}")]
    Instantiation(String),

    /// Store.
    #[error("store: {0}")]
    #[depict(as(depict))]
    Store(#[from] StoreError),

    /// Plugin.
    #[cfg(feature = "plugins")]
    #[error("plugin: {0}")]
    #[depict(as(depict))]
    Plugin(#[from] super::super::plugins::PluginError),

    /// Malformed.
    #[error("malformed: {0}")]
    Malformed(#[from] MalformedError),
}

impl FloriaError {
    /// ID.
    pub fn id(&self) -> Result<Option<ID>, FloriaError> {
        Ok(match self {
            // Self::InvalidValue(invalid_value) => Some(invalid_value.id.clone()),
            #[cfg(feature = "plugins")]
            Self::Plugin(plugin) => match plugin {
                super::super::plugins::PluginError::Dispatch(dispatch) => Some(dispatch.id()?),

                _ => None,
            },

            _ => None,
        })
    }
}

impl IntoDepictionMarkup for FloriaError {
    fn into_depiction_markup(self) -> String {
        match self {
            #[cfg(feature = "plugins")]
            Self::Plugin(error) => error.into_depiction_markup(),
            _ => escape_depiction_markup(self),
        }
    }
}

//
// FloriaErrors
//

/// Floria errors.
pub trait FloriaErrors {
    /// To [Depict].
    fn to_depict(&self, heading: &str) -> DepictFloriaErrors<'_>;
}

impl FloriaErrors for Errors<FloriaError> {
    fn to_depict(&self, heading: &str) -> DepictFloriaErrors<'_> {
        DepictFloriaErrors { heading: heading.into(), errors: self }
    }
}

//
// DepictFloriaErrors
//

/// Depict [FloriaErrors].
pub struct DepictFloriaErrors<'own> {
    heading: String,
    errors: &'own Errors<FloriaError>,
}

impl<'own> Depict for DepictFloriaErrors<'own> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.theme.write_heading(writer, &self.heading)?;

        let mut table = FastHashMap::<_, Vec<_>>::default();
        for error in self.errors {
            let id = error.id().map_err(io::Error::other)?;

            match table.get_mut(&id) {
                Some(list) => list.push(error),

                None => {
                    let mut list = Vec::default();
                    list.push(error);
                    table.insert(id, list);
                }
            }
        }

        for (id, list) in table {
            context.indent(writer)?;

            match id {
                Some(id) => {
                    id.kind.depict(writer, context)?;
                    write!(writer, " ")?;
                    id.depict(writer, context)?;
                }

                None => {
                    context.theme.write_meta(writer, "general")?;
                }
            }

            for (error, last) in IterateWithLast::new(list) {
                context.indent_into_branch(writer, last)?;
                error.depict(
                    writer,
                    &context
                        .child()
                        .with_separator(false)
                        .increase_indentation_branch(last)
                        .with_configuration("variant", "false"),
                )?;
            }
        }

        Ok(())
    }
}
