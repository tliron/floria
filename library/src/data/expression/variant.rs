use super::expression::*;

use compris::normal::*;

impl<AnnotatedT> From<Variant<AnnotatedT>> for Expression {
    fn from(variant: Variant<AnnotatedT>) -> Self {
        match variant {
            Variant::Undefined => Self::Undefined,
            Variant::Null(_) => Self::Null,
            Variant::Integer(integer) => Self::Integer(integer.inner),
            Variant::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(unsigned_integer.inner),
            Variant::Float(float) => Self::Float(float.into()),
            Variant::Boolean(boolean) => Self::Boolean(boolean.inner),
            Variant::Text(text) => Self::Text(text.inner),
            Variant::Blob(blob) => Self::Blob(blob.inner),
            Variant::List(list) => Self::List(list.inner.into_iter().map(|item| item.into()).collect()),
            Variant::Map(map) => {
                Self::Map(map.inner.into_iter().map(|(key, value)| (key.into(), value.into())).collect())
            }
        }
    }
}

// TODO: need a full-blown Clout-like representation?

impl<AnnotatedT> Into<Variant<AnnotatedT>> for Expression
where
    AnnotatedT: Default,
{
    fn into(self) -> Variant<AnnotatedT> {
        match self {
            Self::Undefined => Variant::Undefined,
            Self::Null => Null::default().into(),
            Self::Integer(integer) => integer.into(),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.into(),
            Self::Float(float) => float.into(),
            Self::Boolean(boolean) => boolean.into(),

            Self::Text(text) => {
                // Escape "$"
                if text.starts_with('$') {
                    let text = String::from("$") + &text;
                    text.into()
                } else {
                    text.into()
                }
            }

            Self::Blob(blob) => blob.into(),

            Self::List(list) => list
                .into_iter()
                .map(|item| {
                    let item: Variant<_> = item.into();
                    item
                })
                .collect(),

            Self::Map(map) => map.into_iter().map(|(key, value)| (key.into(), value.into())).collect(),

            Self::Custom(kind, inner) => {
                let mut map = Map::default();
                map.into_insert("$kind", kind);
                map.into_insert("$inner", *inner);
                map.into()
            }

            Self::Call(call) => {
                let mut map = Map::default();
                map.into_insert("$call", call);
                map.into()
            }
        }
    }
}
