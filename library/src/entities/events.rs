use super::super::data::*;

use {kutil::std::immutable::*, std::collections::*};

/// Update event.
pub const UPDATE_EVENT: &str = "floria:update";

/// Added event.
pub const ADDED_EVENT: &str = "floria:added";

/// Before instantiation event.
pub const BEFORE_INSTANTIATION_EVENT: &str = "floria:instantiation:before";

/// Event handlers.
pub type EventHandlers = BTreeMap<ByteString, Vec<FunctionName>>;

//
// AddEventHandler
//

/// Add event handler.
pub trait AddEventHandler {
    /// Add event handler.
    fn add_event_handler(&mut self, event: ByteString, handler: FunctionName);

    /// Add event handler.
    fn add_static_event_handler(&mut self, event: &'static str, handler: FunctionName) {
        self.add_event_handler(ByteString::from_static(event), handler);
    }
}

impl AddEventHandler for EventHandlers {
    fn add_event_handler(&mut self, event: ByteString, handler: FunctionName) {
        self.entry(event).or_default().push(handler);
    }
}

// Utils

/// Event handlers into expression.
pub fn event_handlers_into_expression(map: &mut BTreeMap<Expression, Expression>, event_handlers: EventHandlers) {
    if event_handlers.is_empty() {
        return;
    }

    let mut expressions = BTreeMap::<Expression, Expression>::default();
    for (name, event_handlers) in event_handlers {
        let event_handlers: Vec<Expression> =
            event_handlers.into_iter().map(|function_name| function_name.to_string().into()).collect();
        expressions.insert(name.into(), event_handlers.into());
    }

    map.insert("event-handlers".into(), expressions.into());
}
