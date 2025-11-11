use super::super::{data::*, entities::*};

use {kutil::std::immutable::*, problemo::*};

//
// Store
//

/// Thread-safe access to a Floria store.
///
/// Implementations should ensure that cloning is cheap and clones always refer to the same shared
/// state.
pub trait Store {
    /// Create ID.
    fn create_id(&self, id: &mut ID) -> Result<(), Problem>;

    /// Get plugin.
    fn get_plugin(&self, id: &ID) -> Result<Option<Plugin>, Problem>;

    /// Get plugin by URL.
    fn get_plugin_by_url(&self, url: &ByteString) -> Result<Option<Plugin>, Problem>;

    /// Add plugin.
    fn add_plugin(&self, plugin: Plugin) -> Result<(), Problem>;

    /// Get class.
    fn get_class(&self, id: &ID) -> Result<Option<Class>, Problem>;

    /// Add class.
    fn add_class(&self, class: Class) -> Result<(), Problem>;

    /// Get vertex template.
    fn get_vertex_template(&self, id: &ID) -> Result<Option<VertexTemplate>, Problem>;

    /// Add vertex template.
    ///
    /// Checks to make sure we aren't creating infinite nesting.
    fn add_vertex_template(&self, vertex_template: VertexTemplate) -> Result<(), Problem>;

    /// Get edge template.
    fn get_edge_template(&self, id: &ID) -> Result<Option<EdgeTemplate>, Problem>;

    /// Add edge template.
    fn add_edge_template(&self, edge_template: EdgeTemplate) -> Result<(), Problem>;

    /// Get vertex.
    fn get_vertex(&self, id: &ID) -> Result<Option<Vertex>, Problem>;

    /// Get vertexes.
    fn get_vertexes(&self, directories: Option<Vec<Directory>>) -> Result<Vec<Vertex>, Problem>;

    /// Add vertex.
    fn add_vertex(&self, vertex: Vertex) -> Result<(), Problem>;

    /// Get edge.
    fn get_edge(&self, id: &ID) -> Result<Option<Edge>, Problem>;

    /// Add edge.
    fn add_edge(&self, edge: Edge) -> Result<(), Problem>;
}

//
// StoreUtilities
//

/// Utilities for [Store].
pub trait StoreUtilities {
    /// Get entity as expression.
    fn get_entity_as_expression(&self, id: &ID) -> Result<Option<Expression>, Problem>;
}

impl<StoreT> StoreUtilities for StoreT
where
    StoreT: Clone + Store,
{
    fn get_entity_as_expression(&self, id: &ID) -> Result<Option<Expression>, Problem> {
        let variant = match id.kind {
            EntityKind::Plugin => self.get_plugin(&id)?.map(|plugin| Ok(plugin.into())),
            EntityKind::Class => self.get_class(&id)?.map(|class| Ok(class.into())),
            EntityKind::VertexTemplate => self
                .get_vertex_template(&id)?
                .map(|vertex_template| vertex_template.into_expression(false, self.clone())),
            EntityKind::EdgeTemplate => {
                self.get_edge_template(&id)?.map(|edge_template| edge_template.into_expression(false, self.clone()))
            }
            EntityKind::Vertex => self.get_vertex(&id)?.map(|vertex| vertex.into_expression(false, self.clone())),
            EntityKind::Edge => self.get_edge(&id)?.map(|edge| edge.into_expression(false, self.clone())),
        };

        Ok(match variant {
            Some(variant) => Some(variant?),
            None => None,
        })
    }
}
