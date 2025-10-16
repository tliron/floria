use super::{
    super::{data::*, store::*},
    instance::*,
    utils::*,
};

use {
    kutil::{
        cli::depict::*,
        std::{immutable::*, iter::*},
    },
    std::{collections::*, io},
};

//
// Vertex
//

/// Vertex.
#[derive(Clone, Debug)]
pub struct Vertex {
    /// Instance.
    pub instance: Instance,

    /// Containing vertex ID.
    pub containing_vertex_id: Option<ID>,

    /// Contained vertex IDs.
    pub contained_vertex_ids: Vec<ID>,

    /// Outgoing edges.
    pub outgoing_edge_ids: Vec<ID>,

    /// Incoming edges.
    pub incoming_edge_ids: Vec<ID>,
}

impl Vertex {
    /// Constructor.
    pub fn new<StoreT>(directory: Directory, origin_template_id: ID, store: &StoreT) -> Result<Self, StoreError>
    where
        StoreT: Store,
    {
        let mut id = ID::new(EntityKind::Vertex, directory);
        store.create_id(&mut id)?;
        Ok(Self::new_with(id, Some(origin_template_id)))
    }

    /// Constructor.
    pub fn new_for(directory: Directory, id: ByteString, origin_template_id: Option<ID>) -> Self {
        Self::new_with(ID::new_for(EntityKind::Vertex, directory, id), origin_template_id)
    }

    /// Constructor.
    pub fn new_with(id: ID, origin_template_id: Option<ID>) -> Self {
        Self {
            instance: Instance::new_with(id, origin_template_id),
            containing_vertex_id: None,
            contained_vertex_ids: Default::default(),
            outgoing_edge_ids: Default::default(),
            incoming_edge_ids: Default::default(),
        }
    }

    /// Into expression.
    pub fn into_expression<'own, StoreT>(self, embedded: bool, store: &'own StoreT) -> Result<Expression, StoreError>
    where
        StoreT: Store,
    {
        let mut map = BTreeMap::default();

        self.instance.into_expression(&mut map, embedded, store)?;

        if !embedded {
            if let Some(containing_vertex_id) = &self.containing_vertex_id {
                map.insert("containing_vertex_id".into(), containing_vertex_id.to_string().into());
            }
        }

        if !self.contained_vertex_ids.is_empty() {
            if embedded {
                let mut contained_vertexes = Vec::with_capacity(self.contained_vertex_ids.len());
                for contained_vertex_id in &self.contained_vertex_ids {
                    match store.get_vertex(contained_vertex_id)? {
                        Some(vertex) => contained_vertexes.push(vertex.into_expression(embedded, store)?),
                        None => {}
                    }
                }
                map.insert("contained_vertexes".into(), contained_vertexes.into());
            } else {
                displays_into_expressions(&mut map, "contained_vertex_ids", self.contained_vertex_ids);
            }
        }

        if !self.outgoing_edge_ids.is_empty() {
            if embedded {
                let mut outgoing_edges = Vec::with_capacity(self.outgoing_edge_ids.len());
                for outgoing_edge_id in &self.outgoing_edge_ids {
                    if let Some(edge) = store.get_edge(outgoing_edge_id)? {
                        outgoing_edges.push(edge.into_expression(embedded, store)?);
                    }
                }
                map.insert("outgoing_edges".into(), outgoing_edges.into());
            } else {
                displays_into_expressions(&mut map, "outgoing_edge_ids", self.outgoing_edge_ids);
            }
        }

        if !embedded && !self.incoming_edge_ids.is_empty() {
            displays_into_expressions(&mut map, "incoming_edge_ids", self.incoming_edge_ids);
        }

        Ok(Expression::Map(map))
    }

    /// To [Depict].
    pub fn to_depict<'own, StoreT>(&'own self, store: &'own StoreT) -> DepictVertex<'own, StoreT>
    where
        StoreT: Store,
    {
        DepictVertex { vertex: self, store }
    }
}

//
// DepictVertex
//

/// Depict vertex.
pub struct DepictVertex<'own, StoreT>
where
    StoreT: Store,
{
    vertex: &'own Vertex,
    store: &'own StoreT,
}

impl<'own, StoreT> Depict for DepictVertex<'own, StoreT>
where
    StoreT: Store,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let context = &context.child().with_separator(true);

        context.theme.write_heading(writer, "Vertex")?;
        depict_id("id", Some(&self.vertex.instance.id), false, writer, context)?;
        depict_id("origin_template_id", self.vertex.instance.origin_template_id.as_ref(), false, writer, context)?;
        depict_metadata(&self.vertex.instance.metadata, false, writer, context)?;
        depict_classes(&self.vertex.instance.class_ids, self.store, writer, context)?;
        depict_properties("properties", &self.vertex.instance.properties, self.store, false, writer, context)?;

        utils::depict_field("contained_vertexes", false, writer, context, |writer, context| -> io::Result<()> {
            if self.vertex.contained_vertex_ids.is_empty() {
                context.separate(writer)?;
                context.theme.write_delimiter(writer, "[]")?;
            } else {
                for (vertex_id, last) in IterateWithLast::new(&self.vertex.contained_vertex_ids) {
                    context.indent_into_thick_branch(writer, last)?;
                    match self.store.get_vertex(vertex_id).map_err(io::Error::other)? {
                        Some(vertex) => {
                            vertex
                                .to_depict(self.store)
                                .depict(writer, &context.child().increase_indentation_thick_branch(last))?;
                        }

                        None => {
                            vertex_id.depict(writer, &context.child().with_separator(false))?;
                        }
                    }
                }
            }

            Ok(())
        })?;

        utils::depict_field("outgoing_edges", true, writer, context, |writer, context| -> io::Result<()> {
            if self.vertex.outgoing_edge_ids.is_empty() {
                context.separate(writer)?;
                context.theme.write_delimiter(writer, "[]")?;
            } else {
                for (edge_id, last) in IterateWithLast::new(&self.vertex.outgoing_edge_ids) {
                    context.indent_into_thick_branch(writer, last)?;
                    match self.store.get_edge(edge_id).map_err(io::Error::other)? {
                        Some(edge) => {
                            edge.to_depict(self.store)
                                .depict(writer, &context.child().increase_indentation_thick_branch(last))?;
                        }

                        None => {
                            edge_id.depict(writer, &context.child().with_separator(false))?;
                        }
                    }
                }
            }

            Ok(())
        })
    }
}
