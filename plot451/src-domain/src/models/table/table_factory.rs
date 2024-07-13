use crate::models::column::column_id::ColumnId;

use super::{table::{Table, TableEntityError}, table_name::TableName};
use thiserror::Error;

pub type TableFactoryResult<T> = anyhow::Result<T, TableFactoryError>;
type Result<T> = TableFactoryResult<T>;

pub trait ITableFactory {
    fn create_table(&self, name: TableName, columns: Vec<ColumnId>) -> impl std::future::Future<Output = Result<Table>> + Send;
}

#[derive(Debug, Error)]
pub enum TableFactoryError {
    #[error("Table entity error: [{0}]")]
    TableEntityError(TableEntityError),
    #[error("Unexpected error: [{0}]")]
    Unexpected(Box<dyn std::error::Error + Send + Sync>),
}
