use std::fmt::Display;

use crate::shared::value_object::ValueObject;
use thiserror::Error;

// value object
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct TableId {
    value: String,
}

#[derive(Debug, Error)]
pub enum TableIdError {}

impl ValueObject for TableId {
    type Value = String;
    type Error = TableIdError;

    fn new(value: String) -> Result<Self, TableIdError> {
        Ok(Self { value })
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }

    fn clone_value(&self) -> Self::Value {
        self.value.clone()
    }
}

impl Display for TableId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
