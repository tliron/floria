use super::{
    super::{data::*, store::*},
    template::*,
    *,
};

use {
    depiction::*,
    kutil::std::{immutable::*, iter::*},
    problemo::{common::*, *},
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
    pub fn new(id: ID, containing_vertex_template_id: Option<ID>) -> Self {
        Self {
            template: Template::new(id),
            containing_vertex_template_id,
            contained_vertex_template_ids: Default::default(),
            outgoing_edge_template_ids: Default::default(),
        }
    }

    /// Constructor.
    pub fn new_with_name(
        directory: Directory,
        name: ByteString,
        containing_vertex_template_id: Option<ID>,
    ) -> Result<Self, MalformedError> {
        let id = ID::new_with_name(EntityKind::VertexTemplate, directory, name)?;
        Ok(Self::new(id, containing_vertex_template_id))
    }

    /// Constructor.
    pub fn new_create_id<StoreT>(directory: Directory, store: StoreT) -> Result<Self, Problem>
    where
        StoreT: Store,
    {
        let id = ID::new(EntityKind::VertexTemplate, directory, store)?;
        Ok(Self::new(id, None))
    }

    /// Into expression.
    pub fn into_expression<StoreT>(self, embedded: bool, store: StoreT) -> Result<Expression, Problem>
    where
        StoreT: Clone + Store,
    {
        let mut map = BTreeMap::default();

        self.template.into_expression(&mut map, embedded, store.clone())?;

        if !embedded {
            if let Some(containing_vertex_template_id) = &self.containing_vertex_template_id {
                map.insert("containing-vertex-template-id".into(), containing_vertex_template_id.to_string().into());
            }
        }

        if !self.contained_vertex_template_ids.is_empty() {
            if embedded {
                let mut contained_vertex_templates = Vec::with_capacity(self.contained_vertex_template_ids.len());
                for contained_vertex_template_id in &self.contained_vertex_template_ids {
                    match store.get_vertex_template(contained_vertex_template_id)? {
                        Some(vertex_template) => {
                            contained_vertex_templates.push(vertex_template.into_expression(embedded, store.clone())?)
                        }
                        None => {}
                    }
                }
                map.insert("contained-vertex-templates".into(), contained_vertex_templates.into());
            } else {
                displays_into_expressions(
                    &mut map,
                    "contained-vertex-template-ids",
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
                            outgoing_edge_templates.push(edge_template.into_expression(embedded, store.clone())?)
                        }
                        None => {}
                    }
                }
                map.insert("outgoing-edge-templates".into(), outgoing_edge_templates.into());
            } else {
                displays_into_expressions(&mut map, "outgoing-edge-template-ids", self.outgoing_edge_template_ids);
            }
        }

        Ok(Expression::Map(map))
    }

    /// As [Depict].
    pub fn as_depict<'this, 'store, 'depict, StoreT>(
        &'this self,
        store: &'store StoreT,
    ) -> DepictVertexTemplate<'depict, StoreT>
    where
        'this: 'depict,
        'store: 'depict,
        StoreT: Store,
    {
        DepictVertexTemplate { vertex_template: self, store }
    }
}

//
// DepictVertexTemplate
//

/// Depict vertex template.
pub struct DepictVertexTemplate<'inner, StoreT>
where
    StoreT: Store,
{
    vertex_template: &'inner VertexTemplate,
    store: &'inner StoreT,
}

impl<'inner, StoreT> Depict for DepictVertexTemplate<'inner, StoreT>
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
        depict_event_handlers("event_handlers", &self.vertex_template.template.event_handlers, false, writer, context)?;
        depict_event_handlers(
            "instantiation_event_handlers",
            &self.vertex_template.template.instantiation_event_handlers,
            false,
            writer,
            context,
        )?;

        depict_field("contained_vertex_templates", false, writer, context, |writer, context| -> io::Result<()> {
            if self.vertex_template.contained_vertex_template_ids.is_empty() {
                context.separate(writer)?;
                context.theme.write_delimiter(writer, "[]")?;
            } else {
                for (vertex_template_id, last) in
                    IterateWithLast::new(&self.vertex_template.contained_vertex_template_ids)
                {
                    context.indent_into_thick_branch(writer, last)?;
                    match self.store.get_vertex_template(vertex_template_id).into_io_error()? {
                        Some(vertex_template) => {
                            vertex_template
                                .as_depict(self.store)
                                .depict(writer, &context.child().increase_indentation_thick_branch(last))?;
                        }

                        None => {
                            vertex_template_id.depict(writer, &context.child().with_separator(false))?;
                        }
                    }
                }
            }

            Ok(())
        })?;

        depict_field("outgoing_edge_templates", true, writer, context, |writer, context| -> io::Result<()> {
            if self.vertex_template.outgoing_edge_template_ids.is_empty() {
                context.separate(writer)?;
                context.theme.write_delimiter(writer, "[]")?;
            } else {
                for (edge_template_id, last) in IterateWithLast::new(&self.vertex_template.outgoing_edge_template_ids) {
                    context.indent_into_thick_branch(writer, last)?;
                    match self.store.get_edge_template(edge_template_id).into_io_error()? {
                        Some(edge_template) => {
                            edge_template
                                .as_depict(self.store)
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
