use super::super::{
    super::{data::*, errors::*, plugins::*, store::*},
    vertex_selector::*,
};

use kutil::std::error::*;
impl VertexSelector {
    /// Select.
    pub fn select<StoreT, ErrorReceiverT>(
        &self,
        source_vertex_id: &ID,
        edge_template_id: &ID,
        context: &mut PluginContext<StoreT>,
        errors: &mut ErrorReceiverT,
    ) -> Result<Option<ID>, FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorReceiverT: ErrorReceiver<FloriaError>,
    {
        match self {
            Self::VertexID(id) => Ok(Some(id.clone())),
            Self::Finder(vertex_finder) => vertex_finder.find(source_vertex_id, edge_template_id, context, errors),
        }
    }
}
