use thiserror::Error;

use src_domain::models::column::{column_directory::column_directory_id::{ColumnDirectoryId, ColumnDirectoryIdError}, column_repository::ColumnRepositoryError};

use super::{column_directory_delete_command::ColumnDirectoryDeleteCommand, column_directory_delete_output_data::ColumnDirectoryDeleteOutputData};


pub type ColumnDirectoryDeleteServiceResult<T> = anyhow::Result<T, ColumnDirectoryDeleteServiceError>;

pub trait IColumnDirectoryDeleteService {
    async fn handle(&self, command: ColumnDirectoryDeleteCommand) -> ColumnDirectoryDeleteServiceResult<ColumnDirectoryDeleteOutputData>;
}

#[derive(Debug, Error)]
pub enum ColumnDirectoryDeleteServiceError {
    // repository errors
    #[error("ColumnRepositoryError: [{0}]")]
    ColumnRepositoryError(ColumnRepositoryError),

    // value object errors
    #[error("ColumnDirectoryIdError: [{0}]")]
    ColumnDirectoryIdError(ColumnDirectoryIdError),

    // not found errors
    #[error("Column directory not found, column_id: {0:?}")]
    ColumnDirectoryNotFound(ColumnDirectoryId),
}