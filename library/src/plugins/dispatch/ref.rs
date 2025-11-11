use super::{super::super::store::*, plugin::*};

use std::sync::*;

/// Common reference type for [DispatchPlugin].
pub type DispatchPluginRef<StoreT> = Arc<Mutex<DispatchPlugin<StoreT>>>;

impl<StoreT> From<DispatchPlugin<StoreT>> for DispatchPluginRef<StoreT>
where
    StoreT: Store,
{
    fn from(dispatch: DispatchPlugin<StoreT>) -> Self {
        DispatchPluginRef::new(dispatch.into())
    }
}
