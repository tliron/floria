use super::{
    super::{
        super::{data::*, entities::*},
        store::*,
        wrapper::*,
    },
    backend::*,
};

use {kutil::std::immutable::*, problemo::*, std::sync::*};

//
// InMemoryStore
//

/// In-memory store.
#[derive(Default, Clone)]
pub struct InMemoryStore {
    /// Backend.
    pub backend: Arc<InMemoryStoreBackend>,
}

impl InMemoryStore {
    /// Constructor.
    pub fn new(backend: Arc<InMemoryStoreBackend>) -> StoreWrapper<Self> {
        StoreWrapper::new(Self { backend })
    }
}

impl Store for InMemoryStore {
    fn create_id(&self, id: &mut ID) -> Result<(), Problem> {
        let next_id = self.backend.get_next_id(id.kind.clone());
        id.name = next_id.to_string().into();
        Ok(())
    }

    fn get_plugin(&self, id: &ID) -> Result<Option<Plugin>, Problem> {
        Ok(self.backend.plugins.pin().get(id).cloned())
    }

    fn get_plugin_by_url(&self, url: &ByteString) -> Result<Option<Plugin>, Problem> {
        for plugin in self.backend.plugins.pin().values() {
            if plugin.url == url {
                return Ok(Some(plugin.clone()));
            }
        }
        Ok(None)
    }

    fn add_plugin(&self, plugin: Plugin) -> Result<(), Problem> {
        self.backend.plugins.pin().insert(plugin.id.clone(), plugin);
        Ok(())
    }

    fn get_class(&self, id: &ID) -> Result<Option<Class>, Problem> {
        Ok(self.backend.classes.pin().get(id).cloned())
    }

    fn add_class(&self, class: Class) -> Result<(), Problem> {
        self.backend.classes.pin().insert(class.id.clone(), class);
        Ok(())
    }

    fn get_vertex_template(&self, id: &ID) -> Result<Option<VertexTemplate>, Problem> {
        Ok(self.backend.vertex_templates.pin().get(id).cloned())
    }

    fn add_vertex_template(&self, vertex_template: VertexTemplate) -> Result<(), Problem> {
        self.backend.vertex_templates.pin().insert(vertex_template.template.id.clone(), vertex_template);
        Ok(())
    }

    fn get_edge_template(&self, id: &ID) -> Result<Option<EdgeTemplate>, Problem> {
        Ok(self.backend.edge_templates.pin().get(id).cloned())
    }

    fn add_edge_template(&self, edge_template: EdgeTemplate) -> Result<(), Problem> {
        self.backend.edge_templates.pin().insert(edge_template.template.id.clone(), edge_template);
        Ok(())
    }

    fn get_vertex(&self, id: &ID) -> Result<Option<Vertex>, Problem> {
        Ok(self.backend.vertexes.pin().get(id).cloned())
    }

    fn get_vertexes(&self, _directories: Option<Vec<Directory>>) -> Result<Vec<Vertex>, Problem> {
        Ok(self.backend.vertexes.pin().values().cloned().collect())
    }

    fn add_vertex(&self, vertex: Vertex) -> Result<(), Problem> {
        self.backend.vertexes.pin().insert(vertex.instance.id.clone(), vertex);
        Ok(())
    }

    fn get_edge(&self, id: &ID) -> Result<Option<Edge>, Problem> {
        Ok(self.backend.edges.pin().get(id).cloned())
    }

    fn add_edge(&self, edge: Edge) -> Result<(), Problem> {
        self.backend.edges.pin().insert(edge.instance.id.clone(), edge);
        Ok(())
    }
}
