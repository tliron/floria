use super::{
    super::{
        super::{
            super::{data::*, entities::*, store::*},
            bindings::floria::plugins::floria as bindings,
            errors::*,
        },
        host::*,
    },
    map::Map as BindingsMap,
};

use {kutil::std::immutable::*, std::collections::*, wasmtime::component::Resource};

impl<StoreT> PluginHost<StoreT>
where
    StoreT: Store,
{
    /// Convert a vertex to bindings.
    pub fn vertex_to_bindings(&mut self, vertex: Vertex) -> Result<bindings::Vertex, PluginError> {
        Ok(bindings::Vertex {
            id: vertex.instance.id.into(),
            origin_template_id: id_option_to_bindings(vertex.instance.origin_template_id),
            metadata: self.metadata_to_bindings(vertex.instance.metadata)?,
            class_ids: ids_to_bindings(vertex.instance.class_ids),
            properties: self.properties_to_bindings(vertex.instance.properties)?,
            containing_vertex_id: id_option_to_bindings(vertex.containing_vertex_id),
            contained_vertex_ids: ids_to_bindings(vertex.contained_vertex_ids),
            outgoing_edge_ids: ids_to_bindings(vertex.outgoing_edge_ids),
            incoming_edge_ids: ids_to_bindings(vertex.incoming_edge_ids),
        })
    }

    /// Convert an edge to bindings.
    pub fn edge_to_bindings(&mut self, edge: Edge) -> Result<bindings::Edge, PluginError> {
        Ok(bindings::Edge {
            id: edge.instance.id.into(),
            origin_template_id: id_option_to_bindings(edge.instance.origin_template_id),
            metadata: self.metadata_to_bindings(edge.instance.metadata)?,
            class_ids: ids_to_bindings(edge.instance.class_ids),
            properties: self.properties_to_bindings(edge.instance.properties)?,
            source_vertex_id: edge.source_vertex_id.into(),
            target_vertex_id: edge.target_vertex_id.into(),
        })
    }

    /// Convert metadata to bindings.
    pub fn metadata_to_bindings(&mut self, metadata: Metadata) -> Result<Resource<BindingsMap>, PluginError> {
        let mut key_value_pairs = Vec::with_capacity(metadata.inner.len());
        for (key, value) in metadata {
            key_value_pairs
                .push((self.expression_to_bindings(key.into())?, self.expression_to_bindings(value.into())?));
        }

        Ok(self.resources.push(BindingsMap::new(key_value_pairs))?)
    }

    /// Convert a property to bindings.
    pub fn property_to_bindings(&mut self, property: Property) -> Result<bindings::Property, PluginError> {
        Ok(bindings::Property {
            metadata: self.metadata_to_bindings(property.metadata)?,
            class_ids: ids_to_bindings(property.class_ids),
            read_only: property.read_only,
            preparer: self.expression_option_to_bindings(property.preparer)?,
            updater: self.expression_option_to_bindings(property.updater)?,
            value: self.expression_option_to_bindings(property.value)?,
        })
    }

    fn properties_to_bindings(
        &mut self,
        properties: BTreeMap<ByteString, Property>,
    ) -> Result<Vec<(String, bindings::Property)>, PluginError> {
        let mut property_bindings = Vec::with_capacity(properties.len());

        for (name, property) in properties {
            property_bindings.push((name.into(), self.property_to_bindings(property)?));
        }

        Ok(property_bindings)
    }

    fn expression_option_to_bindings(
        &mut self,
        expression: Option<Expression>,
    ) -> Result<Option<bindings::Expression>, PluginError> {
        Ok(match expression {
            Some(expression) => Some(self.expression_to_bindings(expression)?),
            None => None,
        })
    }
}

fn id_option_to_bindings(id: Option<ID>) -> Option<bindings::Id> {
    id.map(|id| id.into())
}

fn ids_to_bindings(ids: Vec<ID>) -> Vec<bindings::Id> {
    ids.into_iter().map(|id| id.into()).collect()
}
