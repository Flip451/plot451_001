use crate::models::column::column_id::{ColumnId, ColumnIdError};

use super::{
    table::{Table, TableEntityError},
    table_id::{TableId, TableIdError}, table_name::TableNameError,
};
use thiserror::Error;

pub type TableRepositoryResult<T> = anyhow::Result<T, TableRepositoryError>;
type Result<T> = TableRepositoryResult<T>;

pub trait ITableRepository {
    fn save(&self, table: &Table) -> impl std::future::Future<Output = Result<TableId>> + Send;
    fn find(&self, id: &TableId)
        -> impl std::future::Future<Output = Result<Option<Table>>> + Send;
    fn find_parent_table_by_column_id(
        &self,
        column_id: &ColumnId,
    ) -> impl std::future::Future<Output = Result<Vec<Table>>> + Send;
    fn find_all(&self) -> impl std::future::Future<Output = Result<Vec<Table>>> + Send;
    fn delete(&self, table: Table) -> impl std::future::Future<Output = Result<()>> + Send;
}

#[derive(Debug, Error)]
pub enum TableRepositoryError {
    // entity error
    #[error("Table entity error: [{0}]")]
    TableEntityError(TableEntityError),

    // value object error
    #[error("TableId error: [{0}]")]
    TableIdError(TableIdError),
    #[error("TableName error: [{0}]")]
    TableNameError(TableNameError),
    #[error("ColumnId error: [{0}]")]
    ColumnIdError(ColumnIdError),

    #[error("Table not found, table id is {0}")]
    TableNotFound(TableId),
    #[error("Unexpected error: [{0}]")]
    Unexpected(Box<dyn std::error::Error + Send + Sync>),
}
