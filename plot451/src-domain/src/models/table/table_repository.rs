use crate::models::column::column_id::ColumnId;

use super::{
    table::Table,
    table_id::TableId,
};
use thiserror::Error;

pub type TableRepositoryResult<T> = anyhow::Result<T, TableRepositoryError>;
type Result<T> = TableRepositoryResult<T>;

pub trait ITableRepository {
    async fn save(&self, table: &Table) -> Result<TableId>;
    async fn find(&self, id: &TableId) -> Result<Option<Table>>;
    async fn find_parent_table_by_column_id(&self, column_id: &ColumnId) -> Result<Vec<Table>>;
    async fn find_all(&self) -> Result<Vec<Table>>;
    async fn delete(&self, table: Table) -> Result<()>;
}

#[derive(Debug, Error)]
pub enum TableRepositoryError {
    #[error("Table not found, table id is {0}")]
    TableNotFound(TableId),
    #[error("Unexpected error: [{0}]")]
    Unexpected(String),
}
