use super::super::{
    super::{
        super::{errors::*, plugins::*, store::*},
        id::*,
    },
    vertex_selector::*,
};

use kutil::std::error::*;
impl VertexSelector {
    /// Select.
    pub fn select<StoreT, ErrorRecipientT>(
        &self,
        source_vertex_id: &ID,
        edge_template_id: &ID,
        library: &mut Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<ID>, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorRecipientT: ErrorRecipient<FloriaError>,
    {
        match self {
            Self::VertexID(id) => Ok(Some(id.clone())),
            Self::Finder(vertex_finder) => vertex_finder.find(source_vertex_id, edge_template_id, library, errors),
        }
    }
}
