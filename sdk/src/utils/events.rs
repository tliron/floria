use super::super::{data::*, plugin::*};

/// Get the event payload.
pub fn event_payload(arguments: Vec<Expression>, call_site: &CallSite) -> Result<Option<Map>, DispatchError> {
    let mut arguments = arguments.into_iter();
    Ok(
        if let Some(payload) = arguments.next()
            && let Expression::Map(payload) = payload.must_evaluate(call_site)?
        {
            Some(payload.into_map())
        } else {
            None
        },
    )
}
