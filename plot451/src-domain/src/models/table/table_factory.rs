use super::table::TableEntityError;
use thiserror::Error;

pub type TableFactoryResult<T> = anyhow::Result<T, TableFactoryError>;

pub trait ITableFactory {}

#[derive(Debug, Error)]
pub enum TableFactoryError {
    #[error("Table entity error: [{0}]")]
    TableEntityError(TableEntityError),
    #[error("Unexpected error: [{0}]")]
    Unexpected(Box<dyn std::error::Error + Send + Sync>),
}
