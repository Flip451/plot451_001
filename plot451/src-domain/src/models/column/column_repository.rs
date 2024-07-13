use thiserror::Error;

use super::{
    column::Column,
    column_cell::{column_cell::ColumnCell, column_cell_id::ColumnCellId},
    column_directory::{column_directory::ColumnDirectory, column_directory_id::ColumnDirectoryId},
    column_id::ColumnId,
};

pub type ColumnRepositoryResult<T> = anyhow::Result<T, ColumnRepositoryError>;
type Result<T> = ColumnRepositoryResult<T>;

pub trait IColumnRepository {
    fn save(&self, column: &Column) -> impl std::future::Future<Output = Result<ColumnId>> + Send;
    fn find(&self, id: &ColumnId) -> impl std::future::Future<Output = Result<Option<Column>>> + Send;
    fn find_by_ids(&self, ids: &Vec<ColumnId>) -> impl std::future::Future<Output = Result<Vec<Column>>> + Send;
    fn find_by_directory_id(&self, directory_id: &ColumnDirectoryId) -> impl std::future::Future<Output = Result<Vec<Column>>> + Send;
    fn find_all(&self) -> impl std::future::Future<Output = Result<Vec<Column>>> + Send;
    fn delete(&self, column: Column) -> impl std::future::Future<Output = Result<()>> + Send;
    fn save_cell(&self, cell: &ColumnCell) -> impl std::future::Future<Output = Result<ColumnCellId>> + Send;
    fn find_cell(&self, id: &ColumnCellId) -> impl std::future::Future<Output = Result<Option<ColumnCell>>> + Send;
    fn find_cells_by_column_id(&self, column_id: &ColumnId) -> impl std::future::Future<Output = Result<Vec<ColumnCell>>> + Send;
    fn find_cells_by_ids(&self, ids: &Vec<ColumnCellId>) -> impl std::future::Future<Output = Result<Vec<ColumnCell>>> + Send;
    fn delete_cell(&self, cell: ColumnCell) -> impl std::future::Future<Output = Result<()>> + Send;
    fn save_directory(&self, directory: &ColumnDirectory) -> impl std::future::Future<Output = Result<ColumnDirectoryId>> + Send;
    fn find_directory(&self, id: &ColumnDirectoryId) -> impl std::future::Future<Output = Result<Option<ColumnDirectory>>> + Send;
    fn find_children_directories(&self, parent_id: &ColumnDirectoryId) -> impl std::future::Future<Output = Result<Vec<ColumnDirectory>>> + Send;
    fn delete_directory(&self, directory: ColumnDirectory) -> impl std::future::Future<Output = Result<()>> + Send;
}

#[derive(Debug, Error)]
pub enum ColumnRepositoryError {
    #[error("Not all columns found")]
    NotAllColumnsFound(Vec<ColumnId>),
    #[error("Not all cells found")]
    NotAllCellsFound(Vec<ColumnCellId>),
    #[error("Column not found")]
    ColumnNotFound(ColumnId),
    #[error("Unexpected error: [{0}]")]
    Unexpected(Box<dyn std::error::Error + Send + Sync>),
}
