use super::{
    super::super::{super::super::dispatch_bindings::*, custom::*},
    error::*,
    expression::*,
};

use serde::ser::*;

//
// ListSerializer
//

/// List serializer.
#[derive(Debug)]
pub struct ListSerializer<'ser> {
    /// Serializer.
    pub serializer: &'ser ExpressionSerializer,

    /// Custom kind.
    pub custom_kind: Option<&'static str>,

    /// List.
    pub list: Vec<Expression>,
}

impl<'ser> ListSerializer<'ser> {
    /// Constructor.
    pub fn new(
        serializer: &'ser ExpressionSerializer,
        custom_kind: Option<&'static str>,
        length: Option<usize>,
    ) -> Self {
        Self {
            serializer,
            custom_kind,
            list: match length {
                Some(length) => Vec::with_capacity(length),
                None => Vec::default(),
            },
        }
    }
}

impl<'ser> Into<Expression> for ListSerializer<'ser> {
    fn into(self) -> Expression {
        match self.custom_kind {
            Some(custom_kind) => {
                let custom_kind = self.serializer.with_custom_prefix(custom_kind);
                Custom::new(custom_kind, self.list.into()).into()
            }
            None => self.list.into(),
        }
    }
}

impl<'ser> SerializeSeq for ListSerializer<'ser> {
    type Ok = Expression;
    type Error = SerializeError;

    fn serialize_element<SerializeT>(&mut self, element: &SerializeT) -> Result<(), Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        let element = element.serialize(self.serializer)?;
        self.list.push(element);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.into())
    }
}

impl<'ser> SerializeTuple for ListSerializer<'ser> {
    type Ok = Expression;
    type Error = SerializeError;

    fn serialize_element<SerializeT>(&mut self, element: &SerializeT) -> Result<(), Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        let element = element.serialize(self.serializer)?;
        self.list.push(element);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.into())
    }
}

impl<'ser> SerializeTupleStruct for ListSerializer<'ser> {
    type Ok = Expression;
    type Error = SerializeError;

    fn serialize_field<SerializeT>(&mut self, field: &SerializeT) -> Result<(), Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        let field = field.serialize(self.serializer)?;
        self.list.push(field);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.into())
    }
}

impl<'ser> SerializeTupleVariant for ListSerializer<'ser> {
    type Ok = Expression;
    type Error = SerializeError;

    fn serialize_field<SerializeT>(&mut self, field: &SerializeT) -> Result<(), Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        let field = field.serialize(self.serializer)?;
        self.list.push(field);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.into())
    }
}
