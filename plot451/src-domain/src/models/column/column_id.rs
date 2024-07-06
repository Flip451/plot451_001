use std::fmt::Display;

use crate::shared::value_object::ValueObject;
use thiserror::Error;

// value object
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct ColumnId {
    value: String,
}

#[derive(Debug, Error)]
pub enum ColumnIdError {}

impl ValueObject for ColumnId {
    type Value = String;
    type Error = ColumnIdError;

    fn new(value: String) -> Result<Self, ColumnIdError> {
        Ok(Self { value })
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }

    fn clone_value(&self) -> Self::Value {
        self.value.clone()
    }
}

impl Display for ColumnId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}