use super::super::{
    super::data::*,
    bindings::{exports::floria::plugins::dispatch, floria::plugins::floria as host},
};

impl From<host::Site> for dispatch::Site {
    fn from(site: host::Site) -> Self {
        Self::new(site.id.into(), site.path)
    }
}

impl From<dispatch::Site> for host::Site {
    fn from(site: dispatch::Site) -> Self {
        let id: ID = site.id.into();
        Self { id: id.into(), path: site.path }
    }
}
