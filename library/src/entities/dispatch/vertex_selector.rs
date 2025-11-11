use super::super::{
    super::{data::*, plugins::*, store::*},
    vertex_selector::*,
};

use problemo::*;

impl VertexSelector {
    /// Select.
    pub fn select<StoreT, ProblemReceiverT>(
        &self,
        source_vertex_id: &ID,
        edge_template_id: &ID,
        context: &mut PluginContext<StoreT>,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<ID>, Problem>
    where
        StoreT: Clone + Send + Store,
        ProblemReceiverT: ProblemReceiver,
    {
        match self {
            Self::VertexID(id) => Ok(Some(id.clone())),
            Self::Finder(vertex_finder) => vertex_finder.find(source_vertex_id, edge_template_id, context, problems),
        }
    }
}
