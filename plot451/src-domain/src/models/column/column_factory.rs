use thiserror::Error;

pub type ColumnFactoryResult<T> = anyhow::Result<T, ColumnFactoryError>;

pub trait IColumnFactory {}

#[derive(Debug, Error)]
pub enum ColumnFactoryError {
    #[error("Unexpected error: [{0}]")]
    Unexpected(Box<dyn std::error::Error + Send + Sync>),
}
