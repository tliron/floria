use super::super::{
    super::{data::*, errors::*, plugins::*, store::*},
    vertex_finder::*,
};

use kutil::std::error::*;

impl VertexFinder {
    /// Find.
    pub fn find<StoreT, ErrorReceiverT>(
        &self,
        source_vertex_id: &ID,
        _edge_template_id: &ID,
        context: &mut PluginContext<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<Option<ID>, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        let call_site = CallSite::new(source_vertex_id.clone(), Default::default());
        Ok(match self.finder.clone().dispatch(&call_site, context, errors)? {
            Some(id) => match id {
                Expression::Text(id) => Some(ID::parse(EntityKind::Vertex, &id)?),
                _ => None,
            },
            None => None,
        })
    }
}
