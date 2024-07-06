use std::fmt::Display;

use crate::shared::value_object::ValueObject;
use thiserror::Error;

// value object
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct ColumnName {
    value: String,
}

#[derive(Debug, Error)]
pub enum ColumnNameError {
    #[error("Column name is empty.")]
    EmptyNameError,
}

impl ValueObject for ColumnName {
    type Value = String;
    type Error = ColumnNameError;

    fn new(value: String) -> Result<Self, ColumnNameError> {
        let value = value.trim();
        if value.is_empty() {
            return Err(ColumnNameError::EmptyNameError);
        }
        Ok(Self {
            value: value.to_string(),
        })
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }

    fn clone_value(&self) -> Self::Value {
        self.value.clone()
    }
}

impl Display for ColumnName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
