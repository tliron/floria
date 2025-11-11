use super::{
    super::{data::*, entities::*},
    errors::*,
    store::*,
};

use {kutil::std::immutable::*, std::sync::*};

//
// StoreRef
//

/// Common reference type for [Store].
pub type StoreRef = Arc<Box<dyn Store>>;

//
// IntoStoreRef
//

/// Into store reference.
pub trait IntoStoreRef {
    /// Into store reference.
    fn into_ref(self) -> StoreRef;
}

impl<StoreT> IntoStoreRef for StoreT
where
    StoreT: 'static + Store,
{
    fn into_ref(self) -> StoreRef {
        StoreRef::new(Box::new(self))
    }
}

impl Store for StoreRef {
    fn create_id(&self, id: &mut ID) -> Result<(), StoreError> {
        self.as_ref().create_id(id)
    }

    fn get_plugin(&self, id: &ID) -> Result<Option<Plugin>, StoreError> {
        self.as_ref().get_plugin(id)
    }

    fn get_plugin_by_url(&self, url: &ByteString) -> Result<Option<Plugin>, StoreError> {
        self.as_ref().get_plugin_by_url(url)
    }

    fn add_plugin(&self, plugin: Plugin) -> Result<(), StoreError> {
        self.as_ref().add_plugin(plugin)
    }

    fn get_class(&self, id: &ID) -> Result<Option<Class>, StoreError> {
        self.as_ref().get_class(id)
    }

    fn add_class(&self, class: Class) -> Result<(), StoreError> {
        self.as_ref().add_class(class)
    }

    fn get_vertex_template(&self, id: &ID) -> Result<Option<VertexTemplate>, StoreError> {
        self.as_ref().get_vertex_template(id)
    }

    fn add_vertex_template(&self, vertex_template: VertexTemplate) -> Result<(), StoreError> {
        self.as_ref().add_vertex_template(vertex_template)
    }

    fn get_edge_template(&self, id: &ID) -> Result<Option<EdgeTemplate>, StoreError> {
        self.as_ref().get_edge_template(id)
    }

    fn add_edge_template(&self, edge_template: EdgeTemplate) -> Result<(), StoreError> {
        self.as_ref().add_edge_template(edge_template)
    }

    fn get_vertex(&self, id: &ID) -> Result<Option<Vertex>, StoreError> {
        self.as_ref().get_vertex(id)
    }

    fn get_vertexes(&self, directories: Option<Vec<Directory>>) -> Result<Vec<Vertex>, StoreError> {
        self.as_ref().get_vertexes(directories)
    }

    fn add_vertex(&self, vertex: Vertex) -> Result<(), StoreError> {
        self.as_ref().add_vertex(vertex)
    }

    fn get_edge(&self, id: &ID) -> Result<Option<Edge>, StoreError> {
        self.as_ref().get_edge(id)
    }

    fn add_edge(&self, edge: Edge) -> Result<(), StoreError> {
        self.as_ref().add_edge(edge)
    }
}
