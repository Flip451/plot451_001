use crate::shared::value_object::ValueObject;
use thiserror::Error;

// value object
#[derive(Debug, Clone, Eq, Hash)]
pub struct ColumnCellId {
    value: String,
}

#[derive(Debug, Error)]
pub enum ColumnCellIdError {}

impl ValueObject for ColumnCellId {
    type Value = String;

    type Error = ColumnCellIdError;

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

impl PartialEq for ColumnCellId {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
