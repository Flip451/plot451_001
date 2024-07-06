use crate::models::column::column_id::ColumnId;

use super::{table::{Table, TableEntityError}, table_name::TableName};
use thiserror::Error;

pub type TableFactoryResult<T> = anyhow::Result<T, TableFactoryError>;
type Result<T> = TableFactoryResult<T>;

pub trait ITableFactory {
    async fn create_table(&self, name: TableName, columns: Vec<ColumnId>) -> Result<Table>;
}

#[derive(Debug, Error)]
pub enum TableFactoryError {
    #[error("Table entity error: [{0}]")]
    TableEntityError(TableEntityError),
    #[error("Unexpected error: [{0}]")]
    Unexpected(String),
}
