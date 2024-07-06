use thiserror::Error;

use crate::shared::value_object::ValueObject;

// value object
#[derive(Debug, Clone, Eq, Hash)]
pub struct ColumnDirectoryId {
    value: String,
}

#[derive(Debug, Error)]
pub enum ColumnDirectoryIdError {}

impl ValueObject for ColumnDirectoryId {
    type Value = String;

    type Error = ColumnDirectoryIdError;

    fn new(value: Self::Value) -> Result<Self, Self::Error> {
        Ok(Self { value })
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }

    fn clone_value(&self) -> Self::Value {
        self.value.clone()
    }
}

impl PartialEq for ColumnDirectoryId {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}