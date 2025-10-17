use super::{
    super::{data::*, store::*},
    instance::*,
    utils::*,
};

use {
    depiction::*,
    std::{collections::*, io},
};

//
// Edge
//

/// Edge.
#[derive(Clone, Debug)]
pub struct Edge {
    /// Instance.
    pub instance: Instance,

    /// Source vertex ID.
    pub source_vertex_id: ID,

    /// Target vertex ID.
    pub target_vertex_id: ID,
}

impl Edge {
    /// Into expression.
    pub fn into_expression<'own, StoreT>(self, embedded: bool, store: &'own StoreT) -> Result<Expression, StoreError>
    where
        StoreT: Store,
    {
        let mut map = BTreeMap::default();

        self.instance.into_expression(&mut map, embedded, store)?;

        if !embedded {
            map.insert("source-vertex-id".into(), self.source_vertex_id.to_string().into());
        }

        map.insert("target-vertex-id".into(), self.target_vertex_id.to_string().into());

        Ok(map.into())
    }

    /// To [Depict].
    pub fn to_depict<'own, StoreT>(&'own self, store: &'own StoreT) -> DepictEdge<'own, StoreT>
    where
        StoreT: Store,
    {
        DepictEdge { edge: self, store }
    }
}

//
// DepictEdge
//

/// Depict edge.
pub struct DepictEdge<'own, StoreT>
where
    StoreT: Store,
{
    edge: &'own Edge,
    store: &'own StoreT,
}

impl<'own, StoreT> Depict for DepictEdge<'own, StoreT>
where
    StoreT: Store,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.theme.write_heading(writer, "Edge")?;
        depict_id("id", Some(&self.edge.instance.id), false, writer, context)?;
        depict_id("origin_template_id", self.edge.instance.origin_template_id.as_ref(), false, writer, context)?;
        depict_metadata(&self.edge.instance.metadata, false, writer, context)?;
        depict_classes(&self.edge.instance.class_ids, self.store, writer, context)?;
        depict_properties("properties", &self.edge.instance.properties, self.store, false, writer, context)?;
        depict_id("target_vertex_id", Some(&self.edge.target_vertex_id), true, writer, context)
    }
}
