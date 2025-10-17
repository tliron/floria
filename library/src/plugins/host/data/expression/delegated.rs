use super::super::super::super::bindings::floria::plugins::floria as bindings;

use wasmtime::component::Resource;

impl Clone for bindings::Expression {
    fn clone(&self) -> bindings::Expression {
        match self {
            Self::Null => Self::Null,
            Self::Integer(integer) => Self::Integer(*integer),
            Self::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(*unsigned_integer),
            Self::Float(float) => Self::Float(*float),
            Self::Boolean(boolean) => Self::Boolean(*boolean),
            Self::Text(text) => Self::Text(text.clone()),
            Self::Blob(blob) => Self::Blob(blob.clone()),
            // TODO: own or borrow?
            Self::List(resource) => Self::List(Resource::new_own(resource.rep())),
            Self::Map(resource) => Self::Map(Resource::new_own(resource.rep())),
            Self::Custom(resource) => Self::Custom(Resource::new_own(resource.rep())),
            Self::Call(resource) => Self::Call(Resource::new_own(resource.rep())),
        }
    }
}
