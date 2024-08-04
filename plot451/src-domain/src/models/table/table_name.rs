use crate::shared::value_object::ValueObject;
use thiserror::Error;

// value object
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct TableName {
    value: String,
}

#[derive(Debug, Error)]
pub enum TableNameError {
    #[error("Table name is empty.")]
    EmptyNameError,
    #[error("Table name is too long.")]
    TooLongNameError,
}

impl ValueObject for TableName {
    type Value = String;
    type Error = TableNameError;

    fn new(value: String) -> Result<Self, TableNameError> {
        let value = value.trim();
        if value.is_empty() {
            return Err(TableNameError::EmptyNameError);
        }
        if value.len() > 100 {
            return Err(TableNameError::TooLongNameError);
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
