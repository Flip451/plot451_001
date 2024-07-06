use thiserror::Error;

use super::{
    column::Column,
    column_cell::{
        column_cell::ColumnCell, column_cell_id::ColumnCellId, column_cell_value::ColumnCellValue,
    },
    column_directory::{
        column_directory::ColumnDirectory, column_directory_id::ColumnDirectoryId,
        column_directory_name::ColumnDirectoryName,
    },
    column_name::ColumnName,
};

pub type ColumnFactoryResult<T> = anyhow::Result<T, ColumnFactoryError>;

pub trait IColumnFactory {
    async fn create_column(
        &self,
        name: ColumnName,
        directory: ColumnDirectoryId,
        cells: Vec<ColumnCellId>,
    ) -> ColumnFactoryResult<Column>;
    async fn create_cell(&self, value: ColumnCellValue) -> ColumnFactoryResult<ColumnCell>;
    async fn create_directory(
        &self,
        name: ColumnDirectoryName,
        parent_id: Option<ColumnDirectoryId>,
    ) -> ColumnFactoryResult<ColumnDirectory>;
}

#[derive(Debug, Error)]
pub enum ColumnFactoryError {
    #[error("Unexpected error: [{0}]")]
    Unexpected(String),
}
