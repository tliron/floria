use super::super::data::*;

use {kutil::std::immutable::*, std::collections::*};

/// Event handlers.
pub type EventHandlers = BTreeMap<ByteString, Vec<FunctionName>>;
