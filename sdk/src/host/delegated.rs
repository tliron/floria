use super::super::floria_bindings::*;

// Id

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        (self.kind == other.kind) && (self.directory == other.directory) && (self.name == other.name)
    }
}

impl Eq for Id {}

// EntityKind

impl PartialEq for EntityKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Class, Self::Class)
            | (Self::VertexTemplate, Self::VertexTemplate)
            | (Self::EdgeTemplate, Self::EdgeTemplate)
            | (Self::Vertex, Self::Vertex)
            | (Self::Edge, Self::Edge) => true,
            _ => false,
        }
    }
}

impl Eq for EntityKind {}

// Expression

// impl Clone for Expression {
//     fn clone(&self) -> Self {
//         match self {
//             Self::Null => Self::Null,
//             Self::Integer(integer) => Self::Integer(*integer),
//             Self::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(*unsigned_integer),
//             Self::Float(float) => Self::Float(*float),
//             Self::Boolean(boolean) => Self::Boolean(*boolean),
//             Self::Text(text) => Self::Text(text.clone()),
//             Self::Blob(blob) => Self::Blob(blob.clone()),
//             Self::List(list_resource) => Self::List(ListResource::new(list_resource.into_inner())),
//             Self::Map(map_resource) => Self::Map(MapResource::new(map_resource.into_inner())),
//             Self::Custom(custom_resource) => {
//                 let (kind, inner) = custom_resource.into_inner();
//                 Self::Custom(CustomResource::new(&kind, inner))
//             }
//             Self::Call(call_resource) => {
//                 let (plugin, function, arguments, kind) = call_resource.into_inner();
//                 Self::Call(CallResource::new(&plugin, &function, arguments, kind))
//             }
//         }
//     }
// }

// impl PartialEq for Expression {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Self::Null, Self::Null) => true,
//             (Self::Integer(integer), Self::Integer(other_integer)) => integer == other_integer,
//             (Self::UnsignedInteger(unsigned_integer), Self::UnsignedInteger(other_unsigned_integer)) => {
//                 unsigned_integer == other_unsigned_integer
//             }
//             (Self::Float(float), Self::Float(other_float)) => float == other_float,
//             (Self::Boolean(boolean), Self::Boolean(other_boolean)) => boolean == other_boolean,
//             (Self::Text(text), Self::Text(other_text)) => text == other_text,
//             (Self::Blob(blob), Self::Blob(other_blob)) => blob == other_blob,
//             (Self::List(list_resource), Self::List(other_list_resource)) => {
//                 list_resource.inner() == other_list_resource.inner()
//             }
//             (Self::Map(map_resource), Self::Map(other_map_resource)) => {
//                 map_resource.inner() == other_map_resource.inner()
//             }
//             (Self::Custom(custom_resource), Self::Custom(other_custom_resource)) => {
//                 custom_resource.inner() == other_custom_resource.inner()
//             }
//             (Self::Call(call_resource), Self::Call(other_call_resource)) => {
//                 call_resource.inner() == other_call_resource.inner()
//             }

//             _ => false,
//         }
//     }
// }

// impl Eq for Expression {}

// CallKind

// impl PartialEq for CallKind {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Self::Normal, Self::Normal) | (Self::Normal, Self::Eager) | (Self::Normal, Self::Lazy) => true,
//             _ => false,
//         }
//     }
// }

// impl Eq for CallKind {}
