use super::super::{
    super::{data::*, errors::*, plugins::*, store::*},
    vertex_finder::*,
};

use kutil::std::error::*;

impl VertexFinder {
    /// Find.
    pub fn find<StoreT, ErrorRecipientT>(
        &self,
        source_vertex_id: &ID,
        _edge_template_id: &ID,
        library: &mut Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<ID>, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorRecipientT: ErrorRecipient<FloriaError>,
    {
        let call_site = CallSite::new(source_vertex_id.clone(), Default::default());
        Ok(self.finder.dispatch(&call_site, library, errors)?.and_then(|id| match id {
            Expression::Text(id) => Some(ID::parse(EntityKind::Vertex, &id)),
            _ => None,
        }))
    }
}
