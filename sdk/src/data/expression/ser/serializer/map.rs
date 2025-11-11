use super::{
    super::super::{super::super::dispatch_bindings::*, custom::*},
    error::*,
    expression::*,
};

use {
    serde::ser::*,
    std::{collections::*, mem::*},
};

//
// MapSerializer
//

/// Map serializer.
#[derive(Debug)]
pub struct MapSerializer<'ser> {
    /// Serializer.
    pub serializer: &'ser ExpressionSerializer,

    /// Custom kind.
    pub custom_kind: Option<&'static str>,

    /// Map.
    pub map: BTreeMap<Expression, Expression>,

    /// Current key.
    pub key: Option<Expression>,
}

impl<'ser> MapSerializer<'ser> {
    /// Constructor.
    pub fn new(serializer: &'ser ExpressionSerializer, custom_kind: Option<&'static str>) -> Self {
        Self { serializer, custom_kind, map: Default::default(), key: None }
    }
}

impl<'ser> Into<Expression> for MapSerializer<'ser> {
    fn into(self) -> Expression {
        match self.custom_kind {
            Some(custom_kind) => {
                let custom_kind = self.serializer.with_custom_prefix(custom_kind);
                Custom::new(custom_kind, self.map.into()).into()
            }
            None => self.map.into(),
        }
    }
}

impl<'ser> SerializeMap for MapSerializer<'ser> {
    type Ok = Expression;
    type Error = SerializeError;

    fn serialize_key<SerializeT>(&mut self, key: &SerializeT) -> Result<(), Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        if self.key.is_some() {
            return Err(SerializeError("key already serialized".into()));
        }
        let key = key.serialize(self.serializer)?;
        self.key = Some(key);
        Ok(())
    }

    fn serialize_value<SerializeT>(&mut self, value: &SerializeT) -> Result<(), Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        match take(&mut self.key) {
            Some(key) => {
                let value = value.serialize(self.serializer)?;
                self.map.insert(key, value);
                Ok(())
            }

            None => Err(SerializeError("missing key".into())),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.into())
    }
}

impl<'ser> SerializeStruct for MapSerializer<'ser> {
    type Ok = Expression;
    type Error = SerializeError;

    fn serialize_field<SerializeT>(&mut self, key: &'static str, value: &SerializeT) -> Result<(), Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        let value = value.serialize(self.serializer)?;
        self.map.insert(Expression::Text(key.into()), value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.into())
    }
}

impl<'ser> SerializeStructVariant for MapSerializer<'ser> {
    type Ok = Expression;
    type Error = SerializeError;

    fn serialize_field<SerializeT>(&mut self, key: &'static str, value: &SerializeT) -> Result<(), Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        let value = value.serialize(self.serializer)?;
        self.map.insert(Expression::Text(key.into()), value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.into())
    }
}
