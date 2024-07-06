use thiserror::Error;

use src_domain::models::column::{
    column_directory::{
        column_directory_id::{ColumnDirectoryId, ColumnDirectoryIdError},
        column_directory_name::ColumnDirectoryNameError,
    },
    column_factory::ColumnFactoryError,
    column_repository::ColumnRepositoryError,
};

use super::{
    column_directory_create_command::ColumnDirectoryCreateCommand,
    column_directory_create_output_data::ColumnDirectoryCreateOutputData,
};

pub type ColumnDirectoryCreateServiceResult<T> =
    anyhow::Result<T, ColumnDirectoryCreateServiceError>;

pub trait IColumnDirectoryCreateService {
    fn handle(
        &self,
        command: ColumnDirectoryCreateCommand,
    ) -> impl std::future::Future<Output = ColumnDirectoryCreateServiceResult<ColumnDirectoryCreateOutputData>> + Send;
}

#[derive(Debug, Error)]
pub enum ColumnDirectoryCreateServiceError {
    // repository errors
    #[error("ColumnRepositoryError: [{0}]")]
    ColumnRepositoryError(ColumnRepositoryError),

    // value object errors
    #[error("ColumnDirectoryNameError: [{0}]")]
    ColumnDirectoryNameError(ColumnDirectoryNameError),
    #[error("ColumnDirectoryIdError: [{0}]")]
    ColumnDirectoryIdError(ColumnDirectoryIdError),

    // factory errors
    #[error("ColumnFactoryError: [{0}]")]
    ColumnDirectoryFactoryError(ColumnFactoryError),

    // not found errors
    #[error("Column directory not found, directory_id: {0:?}")]
    ColumnDirectoryNotFound(ColumnDirectoryId),
}
