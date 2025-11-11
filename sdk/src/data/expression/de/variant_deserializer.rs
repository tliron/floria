use super::{super::super::super::dispatch_bindings::*, deserializer::*, error::*};

use serde::{Deserializer as _, de};

//
// VariantDeserializer
//

pub(crate) struct VariantDeserializer<'de> {
    expression: &'de Expression,
}

impl<'de> VariantDeserializer<'de> {
    pub fn new(expression: &'de Expression) -> Self {
        Self { expression }
    }
}

impl<'de> de::VariantAccess<'de> for VariantDeserializer<'de> {
    type Error = DeserializeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(DeserializeError(format!("incompatible variant: |error|{}|", self.expression.type_name())))
    }

    fn newtype_variant_seed<SeedT>(self, seed: SeedT) -> Result<SeedT::Value, Self::Error>
    where
        SeedT: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut ExpressionDeserializer::new(self.expression))
    }

    fn tuple_variant<VisitorT>(self, len: usize, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        ExpressionDeserializer::new(self.expression).deserialize_tuple(len, visitor)
    }

    fn struct_variant<VisitorT>(
        self,
        fields: &'static [&'static str],
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        ExpressionDeserializer::new(self.expression).deserialize_struct("", fields, visitor)
    }
}
