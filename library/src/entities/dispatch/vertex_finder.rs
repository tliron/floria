use super::super::{
    super::{data::*, plugins::*, store::*},
    vertex_finder::*,
};

use problemo::*;

impl VertexFinder {
    /// Find.
    pub fn find<StoreT, ProblemReceiverT>(
        &self,
        source_vertex_id: &ID,
        _edge_template_id: &ID,
        context: &mut PluginContext<StoreT>,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<ID>, Problem>
    where
        StoreT: Clone + Send + Store,
        ProblemReceiverT: ProblemReceiver,
    {
        let call_site = CallSite::new(source_vertex_id.clone(), Default::default());
        Ok(match self.finder.clone().dispatch(&call_site, context, problems)? {
            Some(id) => match id {
                Expression::Text(id) => Some(ID::parse(EntityKind::Vertex, &id)?),
                _ => None,
            },
            None => None,
        })
    }
}
