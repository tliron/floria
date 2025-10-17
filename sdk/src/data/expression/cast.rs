use super::super::{
    super::{dispatch_bindings::*, *},
    math::*,
};

impl Expression {
    /// Cast into integer.
    pub fn cast_i64(&self, name: &str) -> Result<i64, DispatchError> {
        match self {
            Expression::Integer(integer) => Ok(*integer),
            _ => Err(errors::not_of_types_for(name, self, &["integer"])),
        }
    }

    /// Cast into integer.
    pub fn cast_i64_integer(&self, name: &str) -> Result<i64, DispatchError> {
        match self {
            Expression::Integer(integer) => Ok(*integer),
            Expression::UnsignedInteger(unsigned_integer) => Ok(into_i64(*unsigned_integer)?),
            _ => Err(errors::not_of_types_for(name, self, &["integer", "unsigned integer"])),
        }
    }

    /// Cast into integer.
    pub fn cast_i32_integer(&self, name: &str) -> Result<i32, DispatchError> {
        match self {
            Expression::Integer(integer) => Ok(into_i32(*integer)?),
            Expression::UnsignedInteger(unsigned_integer) => Ok(into_i32(*unsigned_integer)?),
            _ => Err(errors::not_of_types_for(name, self, &["integer", "unsigned integer"])),
        }
    }

    /// Cast into unsigned integer.
    pub fn cast_u64(&self, name: &str) -> Result<u64, DispatchError> {
        match self {
            Expression::UnsignedInteger(unsigned_integer) => Ok(*unsigned_integer),
            _ => Err(errors::not_of_types_for(name, self, &["unsigned integer"])),
        }
    }

    /// Cast into unsigned integer.
    pub fn cast_u64_integer(&self, name: &str) -> Result<u64, DispatchError> {
        match self {
            Expression::Integer(integer) => Ok(into_u64(*integer)?),
            Expression::UnsignedInteger(unsigned_integer) => Ok(*unsigned_integer),
            _ => Err(errors::not_of_types_for(name, self, &["integer", "unsigned integer"])),
        }
    }

    /// Cast into unsigned integer.
    pub fn cast_u32_integer(&self, name: &str) -> Result<u32, DispatchError> {
        match self {
            Expression::Integer(integer) => Ok(into_u32(*integer)?),
            Expression::UnsignedInteger(unsigned_integer) => Ok(into_u32(*unsigned_integer)?),
            _ => Err(errors::not_of_types_for(name, self, &["integer", "unsigned integer"])),
        }
    }

    /// Cast into float.
    pub fn cast_f64(&self, name: &str) -> Result<f64, DispatchError> {
        match self {
            Expression::Float(float) => Ok(*float),
            _ => Err(errors::not_of_types_for(name, self, &["float"])),
        }
    }

    /// Cast into boolean.
    pub fn cast_bool(&self, name: &str) -> Result<bool, DispatchError> {
        match self {
            Expression::Boolean(boolean) => Ok(*boolean),
            _ => Err(errors::not_of_types_for(name, self, &["boolean"])),
        }
    }

    /// Cast into text.
    pub fn cast_string(self, name: &str) -> Result<String, DispatchError> {
        match self {
            Expression::Text(text) => Ok(text),
            _ => Err(errors::not_of_types_for(name, &self, &["string"])),
        }
    }

    /// Cast into text.
    pub fn cast_string_clone(&self, name: &str) -> Result<String, DispatchError> {
        match self {
            Expression::Text(text) => Ok(text.clone()),
            _ => Err(errors::not_of_types_for(name, &self, &["string"])),
        }
    }

    /// Cast into list.
    pub fn cast_list(&self, name: &str) -> Result<&ListResource, DispatchError> {
        match self {
            Expression::List(list_resource) => Ok(list_resource),
            _ => Err(errors::not_of_types_for(name, &self, &["list"])),
        }
    }

    /// Cast into map.
    pub fn cast_map(&self, name: &str) -> Result<&MapResource, DispatchError> {
        match self {
            Expression::Map(map_resource) => Ok(map_resource),
            _ => Err(errors::not_of_types_for(name, &self, &["map"])),
        }
    }

    /// Cast into custom.
    pub fn cast_custom(&self, name: &str) -> Result<&CustomResource, DispatchError> {
        match self {
            Expression::Custom(custom_resource) => Ok(custom_resource),
            _ => Err(errors::not_of_types_for(name, &self, &["custom data type"])),
        }
    }

    /// Cast into call.
    pub fn cast_call(&self, name: &str) -> Result<&CallResource, DispatchError> {
        match self {
            Expression::Call(call_resource) => Ok(call_resource),
            _ => Err(errors::not_of_types_for(name, &self, &["call"])),
        }
    }
}
