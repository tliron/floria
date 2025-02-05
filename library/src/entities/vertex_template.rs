use super::{
    super::{data::*, store::*},
    template::*,
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
// VertexTemplate
//

/// Vertex template.
#[derive(Clone, Debug)]
pub struct VertexTemplate {
    /// Template.
    pub template: Template,

    /// Containing vertex template ID.
    pub containing_vertex_template_id: Option<ID>,

    /// Contained vertex template IDs.
    pub contained_vertex_template_ids: Vec<ID>,

    /// Outgoing edge template IDs.
    pub outgoing_edge_template_ids: Vec<ID>,
}

impl VertexTemplate {
    /// Constructor.
    pub fn new<StoreT>(directory: Directory, store: &StoreT) -> Result<Self, StoreError>
    where
        StoreT: Store,
    {
        let mut id = ID::new(EntityKind::VertexTemplate, directory);
        store.create_id(&mut id)?;
        Ok(Self::new_with(id, None))
    }

    /// Constructor.
    pub fn new_for(directory: Directory, id: ByteString, containing_vertex_template_id: Option<ID>) -> Self {
        Self::new_with(ID::new_for(EntityKind::VertexTemplate, directory, id), containing_vertex_template_id)
    }

    /// Constructor.
    pub fn new_with(id: ID, containing_vertex_template_id: Option<ID>) -> Self {
        Self {
            template: Template::new(id),
            containing_vertex_template_id,
            contained_vertex_template_ids: Default::default(),
            outgoing_edge_template_ids: Default::default(),
        }
    }

    /// Into expression.
    pub fn into_expression<'own, StoreT>(self, embedded: bool, store: &'own StoreT) -> Result<Expression, StoreError>
    where
        StoreT: Store,
    {
        let mut map = BTreeMap::default();

        self.template.into_expression(&mut map, embedded, store)?;

        if !embedded {
            if let Some(containing_vertex_template_id) = &self.containing_vertex_template_id {
                map.insert("containing_vertex_template_id".into(), containing_vertex_template_id.to_string().into());
            }
        }

        if !self.contained_vertex_template_ids.is_empty() {
            if embedded {
                let mut contained_vertex_templates = Vec::with_capacity(self.contained_vertex_template_ids.len());
                for contained_vertex_template_id in &self.contained_vertex_template_ids {
                    match store.get_vertex_template(contained_vertex_template_id)? {
                        Some(vertex_template) => {
                            contained_vertex_templates.push(vertex_template.into_expression(embedded, store)?)
                        }
                        None => {}
                    }
                }
                map.insert("contained_vertex_templates".into(), contained_vertex_templates.into());
            } else {
                displays_into_expressions(
                    &mut map,
                    "contained_vertex_template_ids",
                    self.contained_vertex_template_ids,
                );
            }
        }

        if !self.outgoing_edge_template_ids.is_empty() {
            if embedded {
                let mut outgoing_edge_templates = Vec::with_capacity(self.outgoing_edge_template_ids.len());
                for outgoing_edge_template_id in &self.outgoing_edge_template_ids {
                    match store.get_edge_template(outgoing_edge_template_id)? {
                        Some(edge_template) => {
                            outgoing_edge_templates.push(edge_template.into_expression(embedded, store)?)
                        }
                        None => {}
                    }
                }
                map.insert("outgoing_edge_templates".into(), outgoing_edge_templates.into());
            } else {
                displays_into_expressions(&mut map, "outgoing_edge_template_ids", self.outgoing_edge_template_ids);
            }
        }

        Ok(Expression::Map(map))
    }

    /// To [Depict].
    pub fn to_depict<'own, StoreT>(&'own self, store: &'own StoreT) -> DepictVertexTemplate<'own, StoreT>
    where
        StoreT: Store,
    {
        DepictVertexTemplate { vertex_template: self, store }
    }
}

//
// DepictVertexTemplate
//

/// Depict vertex template.
pub struct DepictVertexTemplate<'own, StoreT>
where
    StoreT: Store,
{
    vertex_template: &'own VertexTemplate,
    store: &'own StoreT,
}

impl<'own, StoreT> Depict for DepictVertexTemplate<'own, StoreT>
where
    StoreT: Store,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let context = &context.child().with_separator(true);

        context.theme.write_heading(writer, "VertexTemplate")?;
        depict_id("id", Some(&self.vertex_template.template.id), false, writer, context)?;
        depict_metadata(&self.vertex_template.template.metadata, false, writer, context)?;
        depict_classes(&self.vertex_template.template.class_ids, self.store, writer, context)?;
        depict_properties(
            "property_templates",
            &self.vertex_template.template.property_templates,
            self.store,
            false,
            writer,
            context,
        )?;

        utils::depict_field(
            "contained_vertex_templates",
            false,
            writer,
            context,
            |writer, context| -> io::Result<()> {
                if self.vertex_template.contained_vertex_template_ids.is_empty() {
                    context.separate(writer)?;
                    context.theme.write_delimiter(writer, "[]")?;
                } else {
                    for (vertex_template_id, last) in
                        IterateWithLast::new(&self.vertex_template.contained_vertex_template_ids)
                    {
                        context.indent_into_thick_branch(writer, last)?;
                        match self.store.get_vertex_template(vertex_template_id).map_err(io::Error::other)? {
                            Some(vertex_template) => {
                                vertex_template
                                    .to_depict(self.store)
                                    .depict(writer, &context.child().increase_indentation_thick_branch(last))?;
                            }

                            None => {
                                vertex_template_id.depict(writer, &context.child().with_separator(false))?;
                            }
                        }
                    }
                }

                Ok(())
            },
        )?;

        utils::depict_field("outgoing_edge_templates", true, writer, context, |writer, context| -> io::Result<()> {
            if self.vertex_template.outgoing_edge_template_ids.is_empty() {
                context.separate(writer)?;
                context.theme.write_delimiter(writer, "[]")?;
            } else {
                for (edge_template_id, last) in IterateWithLast::new(&self.vertex_template.outgoing_edge_template_ids) {
                    context.indent_into_thick_branch(writer, last)?;
                    match self.store.get_edge_template(edge_template_id).map_err(io::Error::other)? {
                        Some(edge_template) => {
                            edge_template
                                .to_depict(self.store)
                                .depict(writer, &context.child().increase_indentation_thick_branch(last))?;
                        }

                        None => {
                            edge_template_id.depict(writer, &context.child().with_separator(false))?;
                        }
                    }
                }
            }

            Ok(())
        })
    }
}
