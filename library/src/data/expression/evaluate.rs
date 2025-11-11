use super::{
    super::super::{plugins, store::*},
    call::*,
    expression::*,
};

use {problemo::*, std::collections::*};

impl Expression {
    /// Evaluate the expression.
    ///
    /// Eager calls will be dispatched here.
    pub fn evaluate<StoreT, ProblemReceiverT>(
        self,
        call_site: &plugins::CallSite,
        context: &mut plugins::PluginContext<StoreT>,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<Expression>, Problem>
    where
        StoreT: Clone + Send + Store,
        ProblemReceiverT: ProblemReceiver,
    {
        match self {
            Self::List(list) => {
                let mut expressions = Vec::with_capacity(list.len());
                for item in list {
                    expressions.push(item.evaluate(call_site, context, problems)?.unwrap_or_default());
                }
                Ok(Some(expressions.into()))
            }

            Self::Map(map) => {
                let mut expressions = BTreeMap::default();
                for (key, value) in map {
                    if let Some(key) = key.evaluate(call_site, context, problems)? {
                        expressions.insert(key, value.evaluate(call_site, context, problems)?.unwrap_or_default());
                    }
                }
                Ok(Some(expressions.into()))
            }

            Self::Call(call) => {
                if matches!(call.kind, CallKind::Eager) {
                    call.dispatch(call_site, context, problems)
                } else {
                    Ok(Some(Self::Call(call)))
                }
            }

            _ => Ok(Some(self)),
        }
    }
}
