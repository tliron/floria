use super::super::{
    super::{
        super::{
            super::{data::*, entities::*, store::*},
            bindings::floria::plugins::floria as bindings,
        },
        host::*,
    },
    conversions::*,
    map::Map as BindingsMap,
};

use {problemo::*, wasmtime::component::Resource};

impl<StoreT> PluginHost<StoreT>
where
    StoreT: Store,
{
    /// Convert a vertex to bindings.
    pub fn vertex_to_bindings(&mut self, vertex: Vertex) -> Result<bindings::Vertex, Problem> {
        Ok(bindings::Vertex {
            id: vertex.instance.id.into(),
            origin_template_id: id_option_to_bindings(vertex.instance.origin_template_id),
            metadata: self.metadata_to_bindings(vertex.instance.metadata)?,
            class_ids: ids_to_bindings(vertex.instance.class_ids),
            properties: self.properties_to_bindings(vertex.instance.properties)?,
            event_handlers: event_handlers_to_bindings(vertex.instance.event_handlers),
            containing_vertex_id: id_option_to_bindings(vertex.containing_vertex_id),
            contained_vertex_ids: ids_to_bindings(vertex.contained_vertex_ids),
            outgoing_edge_ids: ids_to_bindings(vertex.outgoing_edge_ids),
            incoming_edge_ids: ids_to_bindings(vertex.incoming_edge_ids),
        })
    }

    /// Convert a vertex from bindings.
    pub fn vertex_from_bindings(&mut self, vertex: bindings::Vertex) -> Result<Vertex, Problem> {
        Ok(Vertex {
            instance: Instance {
                id: vertex.id.try_into()?,
                origin_template_id: id_option_from_bindings(vertex.origin_template_id)?,
                metadata: self.metadata_from_bindings(vertex.metadata)?,
                class_ids: ids_from_bindings(vertex.class_ids)?,
                properties: self.properties_from_bindings(vertex.properties)?,
                event_handlers: event_handlers_from_bindings(vertex.event_handlers)?,
            },
            containing_vertex_id: id_option_from_bindings(vertex.containing_vertex_id)?,
            contained_vertex_ids: ids_from_bindings(vertex.contained_vertex_ids)?,
            outgoing_edge_ids: ids_from_bindings(vertex.outgoing_edge_ids)?,
            incoming_edge_ids: ids_from_bindings(vertex.incoming_edge_ids)?,
        })
    }

    /// Convert an edge to bindings.
    pub fn edge_to_bindings(&mut self, edge: Edge) -> Result<bindings::Edge, Problem> {
        Ok(bindings::Edge {
            id: edge.instance.id.into(),
            origin_template_id: id_option_to_bindings(edge.instance.origin_template_id),
            metadata: self.metadata_to_bindings(edge.instance.metadata)?,
            class_ids: ids_to_bindings(edge.instance.class_ids),
            properties: self.properties_to_bindings(edge.instance.properties)?,
            event_handlers: event_handlers_to_bindings(edge.instance.event_handlers),
            source_vertex_id: edge.source_vertex_id.into(),
            target_vertex_id: edge.target_vertex_id.into(),
        })
    }

    /// Convert an edge from bindings.
    pub fn edge_from_bindings(&mut self, edge: bindings::Edge) -> Result<Edge, Problem> {
        Ok(Edge {
            instance: Instance {
                id: edge.id.try_into()?,
                origin_template_id: id_option_from_bindings(edge.origin_template_id)?,
                metadata: self.metadata_from_bindings(edge.metadata)?,
                class_ids: ids_from_bindings(edge.class_ids)?,
                properties: self.properties_from_bindings(edge.properties)?,
                event_handlers: event_handlers_from_bindings(edge.event_handlers)?,
            },
            source_vertex_id: edge.source_vertex_id.try_into()?,
            target_vertex_id: edge.target_vertex_id.try_into()?,
        })
    }

    /// Convert a property to bindings.
    pub fn property_to_bindings(&mut self, property: Property) -> Result<bindings::Property, Problem> {
        Ok(bindings::Property {
            metadata: self.metadata_to_bindings(property.metadata)?,
            class_ids: ids_to_bindings(property.class_ids),
            read_only: property.read_only,
            preparer: self.expression_option_to_bindings(property.preparer)?,
            updater: self.expression_option_to_bindings(property.updater)?,
            value: self.expression_option_to_bindings(property.value)?,
        })
    }

    /// Convert a property from bindings.
    pub fn property_from_bindings(&mut self, property: bindings::Property) -> Result<Property, Problem> {
        Ok(Property {
            metadata: self.metadata_from_bindings(property.metadata)?,
            class_ids: ids_from_bindings(property.class_ids)?,
            read_only: property.read_only,
            preparer: self.expression_option_from_bindings(property.preparer)?,
            updater: self.expression_option_from_bindings(property.updater)?,
            value: self.expression_option_from_bindings(property.value)?,
        })
    }

    /// Convert properties from bindings.
    pub fn properties_to_bindings(
        &mut self,
        properties: Properties,
    ) -> Result<Vec<(String, bindings::Property)>, Problem> {
        let mut property_bindings = Vec::with_capacity(properties.len());
        for (name, property) in properties {
            property_bindings.push((name.into(), self.property_to_bindings(property)?));
        }
        Ok(property_bindings)
    }

    /// Convert properties to bindings.
    pub fn properties_from_bindings(
        &mut self,
        properties: Vec<(String, bindings::Property)>,
    ) -> Result<Properties, Problem> {
        let mut floria_properties = Properties::default();
        for (name, property) in properties {
            floria_properties.insert(name.into(), self.property_from_bindings(property)?);
        }
        Ok(floria_properties)
    }

    /// Convert metadata to bindings.
    pub fn metadata_to_bindings(&mut self, metadata: Metadata) -> Result<Resource<BindingsMap>, Problem> {
        let mut key_value_pairs = Vec::with_capacity(metadata.inner.len());
        for (key, value) in metadata {
            key_value_pairs
                .push((self.expression_to_bindings(key.into())?, self.expression_to_bindings(value.into())?));
        }
        Ok(self.resources.push(BindingsMap::new(key_value_pairs))?)
    }

    /// Convert metadata to bindings.
    pub fn metadata_from_bindings(&mut self, metadata: Resource<BindingsMap>) -> Result<Metadata, Problem> {
        let metadata = self.resources.delete(metadata)?;
        let mut floria_metadata = Metadata::default();
        for (key, value) in metadata.inner {
            floria_metadata.into_insert(self.expression_from_bindings(key)?, self.expression_from_bindings(value)?);
        }
        Ok(floria_metadata)
    }
}

/// Convert event handlers to bindings.
pub fn event_handlers_to_bindings(event_handlers: EventHandlers) -> Vec<(String, bindings::Id, String)> {
    let mut event_handler_bindings = Vec::default();
    for (event, handlers) in event_handlers {
        for handler in handlers {
            event_handler_bindings.push((event.clone().into(), handler.plugin_id.into(), handler.name.into()));
        }
    }
    event_handler_bindings
}

/// Convert event handlers from bindings.
pub fn event_handlers_from_bindings(
    event_handlers: Vec<(String, bindings::Id, String)>,
) -> Result<EventHandlers, Problem> {
    let mut floria_event_handlers = EventHandlers::default();
    for (event, plugin_id, function_name) in event_handlers {
        let event = event.into();
        let handler = FunctionName::new(plugin_id.try_into()?, function_name.into())?;
        match floria_event_handlers.get_mut(&event) {
            Some(handlers) => handlers.push(handler),
            None => {
                floria_event_handlers.insert(event, vec![handler]);
            }
        }
    }
    Ok(floria_event_handlers)
}
