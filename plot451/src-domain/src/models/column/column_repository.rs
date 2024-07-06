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
    async fn save(&self, column: &Column) -> Result<ColumnId>;
    async fn find(&self, id: &ColumnId) -> Result<Option<Column>>;
    async fn find_by_ids(&self, ids: &Vec<ColumnId>) -> Result<Vec<Column>>;
    async fn find_by_directory_id(&self, directory_id: &ColumnDirectoryId) -> Result<Vec<Column>>;
    async fn find_all(&self) -> Result<Vec<Column>>;
    async fn delete(&self, column: Column) -> Result<()>;
    async fn save_cell(&self, cell: &ColumnCell) -> Result<ColumnCellId>;
    async fn find_cell(&self, id: &ColumnCellId) -> Result<Option<ColumnCell>>;
    async fn find_cells_by_column_id(&self, column_id: &ColumnId) -> Result<Vec<ColumnCell>>;
    async fn find_cells_by_ids(&self, ids: &Vec<ColumnCellId>) -> Result<Vec<ColumnCell>>;
    async fn delete_cell(&self, cell: ColumnCell) -> Result<()>;
    async fn save_directory(&self, directory: &ColumnDirectory) -> Result<ColumnDirectoryId>;
    async fn find_directory(&self, id: &ColumnDirectoryId) -> Result<Option<ColumnDirectory>>;
    async fn find_children_directories(&self, parent_id: &ColumnDirectoryId) -> Result<Vec<ColumnDirectory>>;
    async fn delete_directory(&self, directory: ColumnDirectory) -> Result<()>;
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
    Unexpected(String),
}
