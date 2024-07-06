use thiserror::Error;

use crate::shared::value_object::ValueObject;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct ColumnDirectoryName {
    value: String,
}

#[derive(Debug, Error)]
pub enum ColumnDirectoryNameError {
    #[error("Column name is empty.")]
    EmptyNameError,
}

impl ValueObject for ColumnDirectoryName {
    type Value = String;

    type Error = ColumnDirectoryNameError;

    fn new(value: Self::Value) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(ColumnDirectoryNameError::EmptyNameError);
        }
        Ok(Self { value: value.to_string() })
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }

    fn clone_value(&self) -> Self::Value {
        self.value.clone()
    }
}