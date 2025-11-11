use super::{
    super::{data::*, entities::*},
    store::*,
};

use {kutil::std::immutable::*, problemo::*, std::sync::*};

//
// StoreRef
//

/// Common reference type for [Store].
pub type StoreRef = Arc<Box<dyn Store>>;

//
// AsStoreRef
//

/// As store reference.
pub trait AsStoreRef {
    /// As store reference.
    fn as_ref(self) -> StoreRef;
}

impl<StoreT> AsStoreRef for StoreT
where
    StoreT: 'static + Store,
{
    fn as_ref(self) -> StoreRef {
        StoreRef::new(Box::new(self))
    }
}

impl Store for StoreRef {
    fn create_id(&self, id: &mut ID) -> Result<(), Problem> {
        self.as_ref().create_id(id)
    }

    fn get_plugin(&self, id: &ID) -> Result<Option<Plugin>, Problem> {
        self.as_ref().get_plugin(id)
    }

    fn get_plugin_by_url(&self, url: &ByteString) -> Result<Option<Plugin>, Problem> {
        self.as_ref().get_plugin_by_url(url)
    }

    fn add_plugin(&self, plugin: Plugin) -> Result<(), Problem> {
        self.as_ref().add_plugin(plugin)
    }

    fn get_class(&self, id: &ID) -> Result<Option<Class>, Problem> {
        self.as_ref().get_class(id)
    }

    fn add_class(&self, class: Class) -> Result<(), Problem> {
        self.as_ref().add_class(class)
    }

    fn get_vertex_template(&self, id: &ID) -> Result<Option<VertexTemplate>, Problem> {
        self.as_ref().get_vertex_template(id)
    }

    fn add_vertex_template(&self, vertex_template: VertexTemplate) -> Result<(), Problem> {
        self.as_ref().add_vertex_template(vertex_template)
    }

    fn get_edge_template(&self, id: &ID) -> Result<Option<EdgeTemplate>, Problem> {
        self.as_ref().get_edge_template(id)
    }

    fn add_edge_template(&self, edge_template: EdgeTemplate) -> Result<(), Problem> {
        self.as_ref().add_edge_template(edge_template)
    }

    fn get_vertex(&self, id: &ID) -> Result<Option<Vertex>, Problem> {
        self.as_ref().get_vertex(id)
    }

    fn get_vertexes(&self, directories: Option<Vec<Directory>>) -> Result<Vec<Vertex>, Problem> {
        self.as_ref().get_vertexes(directories)
    }

    fn add_vertex(&self, vertex: Vertex) -> Result<(), Problem> {
        self.as_ref().add_vertex(vertex)
    }

    fn get_edge(&self, id: &ID) -> Result<Option<Edge>, Problem> {
        self.as_ref().get_edge(id)
    }

    fn add_edge(&self, edge: Edge) -> Result<(), Problem> {
        self.as_ref().add_edge(edge)
    }
}
