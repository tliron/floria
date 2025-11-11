use super::{
    super::{data::*, entities::*},
    errors::*,
    store::*,
};

use {kutil::std::immutable::*, problemo::*};

//
// StoreWrapper
//

/// [Store] wrapper.
#[derive(Clone, Debug)]
pub struct StoreWrapper<StoreT> {
    /// Inner.
    pub inner: StoreT,
}

impl<StoreT> StoreWrapper<StoreT>
where
    StoreT: Store,
{
    /// Constructor.
    pub fn new(inner: StoreT) -> Self {
        Self { inner }
    }
}

impl<StoreT> Store for StoreWrapper<StoreT>
where
    StoreT: Store,
{
    fn create_id(&self, id: &mut ID) -> Result<(), Problem> {
        self.inner.create_id(id)?;
        tracing::debug!(id = id.to_string(), "create_id");
        Ok(())
    }

    fn get_plugin(&self, id: &ID) -> Result<Option<Plugin>, Problem> {
        tracing::debug!(id = id.to_string(), "get_plugin");
        if id.kind != EntityKind::Plugin {
            return Err(WrongKindError::problem(EntityKind::Plugin, id.kind).via(StoreError));
        }
        self.inner.get_plugin(id)
    }

    fn get_plugin_by_url(&self, url: &ByteString) -> Result<Option<Plugin>, Problem> {
        tracing::debug!("get_plugin_by_url");
        self.inner.get_plugin_by_url(url)
    }

    fn add_plugin(&self, plugin: Plugin) -> Result<(), Problem> {
        tracing::debug!(id = plugin.id.to_string(), "add_plugin");
        if plugin.id.kind != EntityKind::Plugin {
            return Err(WrongKindError::problem(EntityKind::Plugin, plugin.id.kind).via(StoreError));
        }
        self.inner.add_plugin(plugin)
    }

    fn get_class(&self, id: &ID) -> Result<Option<Class>, Problem> {
        tracing::debug!(id = id.to_string(), "get_class");
        if id.kind != EntityKind::Class {
            return Err(WrongKindError::problem(EntityKind::Class, id.kind).via(StoreError));
        }
        self.inner.get_class(id)
    }

    fn add_class(&self, class: Class) -> Result<(), Problem> {
        tracing::debug!(id = class.id.to_string(), "add_class");
        if class.id.kind != EntityKind::Class {
            return Err(WrongKindError::problem(EntityKind::Class, class.id.kind).via(StoreError));
        }
        self.inner.add_class(class)
    }

    fn get_vertex_template(&self, id: &ID) -> Result<Option<VertexTemplate>, Problem> {
        tracing::debug!(id = id.to_string(), "get_vertex_template");
        if id.kind != EntityKind::VertexTemplate {
            return Err(WrongKindError::problem(EntityKind::VertexTemplate, id.kind).via(StoreError));
        }
        self.inner.get_vertex_template(id)
    }

    fn add_vertex_template(&self, vertex_template: VertexTemplate) -> Result<(), Problem> {
        tracing::debug!(id = vertex_template.template.id.to_string(), "add_vertex_template");
        if vertex_template.template.id.kind != EntityKind::VertexTemplate {
            return Err(
                WrongKindError::problem(EntityKind::VertexTemplate, vertex_template.template.id.kind).via(StoreError)
            );
        }
        self.inner.add_vertex_template(vertex_template)
    }

    fn get_edge_template(&self, id: &ID) -> Result<Option<EdgeTemplate>, Problem> {
        tracing::debug!(id = id.to_string(), "get_edge_template");
        if id.kind != EntityKind::EdgeTemplate {
            return Err(WrongKindError::problem(EntityKind::EdgeTemplate, id.kind).via(StoreError));
        }
        self.get_edge_template(id)
    }

    fn add_edge_template(&self, edge_template: EdgeTemplate) -> Result<(), Problem> {
        tracing::debug!(id = edge_template.template.id.to_string(), "add_edge_template");
        if edge_template.template.id.kind != EntityKind::EdgeTemplate {
            return Err(
                WrongKindError::problem(EntityKind::EdgeTemplate, edge_template.template.id.kind).via(StoreError)
            );
        }
        self.inner.add_edge_template(edge_template)
    }

    fn get_vertex(&self, id: &ID) -> Result<Option<Vertex>, Problem> {
        tracing::debug!(id = id.to_string(), "get_vertex");
        if id.kind != EntityKind::Vertex {
            return Err(WrongKindError::problem(EntityKind::Vertex, id.kind).via(StoreError));
        }
        self.inner.get_vertex(id)
    }

    fn get_vertexes(&self, directories: Option<Vec<Directory>>) -> Result<Vec<Vertex>, Problem> {
        tracing::debug!("get_vertexes");
        self.inner.get_vertexes(directories)
    }

    fn add_vertex(&self, vertex: Vertex) -> Result<(), Problem> {
        tracing::debug!(id = vertex.instance.id.to_string(), "add_vertex");
        if vertex.instance.id.kind != EntityKind::Vertex {
            return Err(WrongKindError::problem(EntityKind::Vertex, vertex.instance.id.kind).via(StoreError));
        }
        self.inner.add_vertex(vertex)
    }

    fn get_edge(&self, id: &ID) -> Result<Option<Edge>, Problem> {
        tracing::debug!(id = id.to_string(), "get_edge");
        if id.kind != EntityKind::Edge {
            return Err(WrongKindError::problem(EntityKind::Edge, id.kind).via(StoreError));
        }
        self.inner.get_edge(id)
    }

    fn add_edge(&self, edge: Edge) -> Result<(), Problem> {
        tracing::debug!(id = edge.instance.id.to_string(), "add_edge");
        if edge.instance.id.kind != EntityKind::Edge {
            return Err(WrongKindError::problem(EntityKind::Edge, edge.instance.id.kind).via(StoreError));
        }
        self.inner.add_edge(edge)
    }
}
