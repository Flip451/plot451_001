use thiserror::Error;

use src_domain::models::column::{
    column_directory::column_directory_id::{ColumnDirectoryId, ColumnDirectoryIdError},
    column_repository::ColumnRepositoryError,
};

use super::{
    column_directory_contents_list_command::ColumnDirectoryContentsListCommand,
    column_directory_contents_list_output_data::ColumnDirectoryContentsListOutputData,
};

pub type ColumnDirectoryContentsListServiceResult<T> = anyhow::Result<T, ColumnDirectoryContentsListServiceError>;

pub trait IColumnDirectoryContentsListService {
    async fn handle(
        &self,
        command: ColumnDirectoryContentsListCommand,
    ) -> ColumnDirectoryContentsListServiceResult<ColumnDirectoryContentsListOutputData>;
}

#[derive(Debug, Error)]
pub enum ColumnDirectoryContentsListServiceError {
    // repository errors
    #[error("ColumnRepositoryError: [{0}]")]
    ColumnRepositoryError(ColumnRepositoryError),

    // value object errors
    #[error("ColumnDirectoryIdError: [{0}]")]
    ColumnDirectoryIdError(ColumnDirectoryIdError),

    // not found errors
    #[error("Column Directory not found, directory_id: {0:?}")]
    ColumnDirectoryNotFound(ColumnDirectoryId),
}
