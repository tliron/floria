use super::super::{
    super::{data::*, store::*},
    events::*,
    property::*,
};

use {depiction::*, kutil::std::iter::*, problemo::*, std::io};

/// Depict metadata.
pub fn depict_metadata<WriteT>(
    metadata: &Metadata,
    last: bool,
    writer: &mut WriteT,
    context: &DepictionContext,
) -> io::Result<()>
where
    WriteT: io::Write,
{
    depict_field("metadata", last, writer, context, |writer, context| -> io::Result<()> {
        metadata.depict(writer, context)
    })
}

/// Depict ID.
pub fn depict_id<WriteT>(
    name: &str,
    id: Option<&ID>,
    last: bool,
    writer: &mut WriteT,
    context: &DepictionContext,
) -> io::Result<()>
where
    WriteT: io::Write,
{
    depict_field(name, last, writer, context, |writer, context| -> io::Result<()> {
        match id {
            Some(id) => id.depict(writer, context),
            None => {
                context.separate(writer)?;
                context.theme.write_symbol(writer, "None")
            }
        }
    })
}

/// Depict properties.
pub fn depict_properties<StoreT, WriteT>(
    name: &str,
    properties: &Properties,
    store: &StoreT,
    last: bool,
    writer: &mut WriteT,
    context: &DepictionContext,
) -> io::Result<()>
where
    StoreT: Store,
    WriteT: io::Write,
{
    depict_field(name, last, writer, context, |writer, context| -> io::Result<()> {
        if properties.is_empty() {
            context.separate(writer)?;
            context.theme.write_delimiter(writer, "{}")?;
        } else {
            for ((name, property), last) in IterateWithLast::new(properties) {
                context.indent_into_branch(writer, last)?;
                context.theme.write_meta(writer, name)?;
                context.theme.write_delimiter(writer, ':')?;
                property.as_depict(store).depict(writer, &context.child().increase_indentation_branch(last))?;
            }
        }

        Ok(())
    })
}

/// Depict event handlers.
pub fn depict_event_handlers<WriteT>(
    name: &str,
    event_handlers: &EventHandlers,
    last: bool,
    writer: &mut WriteT,
    context: &DepictionContext,
) -> io::Result<()>
where
    WriteT: io::Write,
{
    depict_field(name, last, writer, context, |writer, context| -> io::Result<()> {
        if event_handlers.is_empty() {
            context.separate(writer)?;
            context.theme.write_delimiter(writer, "{}")?;
        } else {
            for ((name, event_handlers), last) in IterateWithLast::new(event_handlers) {
                context.indent_into_branch(writer, last)?;
                context.theme.write_meta(writer, name)?;
                context.theme.write_delimiter(writer, ':')?;
                let child_context = &context.child().increase_indentation_branch(last);
                for event_handler in event_handlers {
                    child_context.indent_into(writer, DEPICT_INTO_LIST_ITEM)?;
                    event_handler.depict(writer, child_context)?;
                }
            }
        }

        Ok(())
    })
}

/// Depict classes.
pub fn depict_classes<WriteT, StoreT>(
    class_ids: &Vec<ID>,
    store: &StoreT,
    writer: &mut WriteT,
    context: &DepictionContext,
) -> io::Result<()>
where
    WriteT: io::Write,
    StoreT: Store,
{
    depict_field("classes", false, writer, context, |writer, context| -> io::Result<()> {
        if class_ids.is_empty() {
            context.separate(writer)?;
            context.theme.write_delimiter(writer, "[]")?;
        } else {
            for (class_id, last) in IterateWithLast::new(class_ids) {
                context.indent_into_double_branch(writer, last)?;
                match store.get_class(class_id).into_io_error()? {
                    Some(class) => {
                        class
                            .as_depict(store)
                            .depict(writer, &context.child().increase_indentation_double_branch(last))?;
                    }

                    None => {
                        class_id.depict(writer, &context.child().with_separator(false))?;
                    }
                }
            }
        }

        Ok(())
    })
}
