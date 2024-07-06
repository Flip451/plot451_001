use thiserror::Error;

use src_domain::models::column::{
    column_cell::column_cell_value::ColumnCellValueError,
    column_directory::column_directory_id::ColumnDirectoryIdError,
    column_factory::ColumnFactoryError, column_id::ColumnIdError, column_name::ColumnNameError,
    column_repository::ColumnRepositoryError,
};

use super::{
    column_create_command::ColumnCreateCommand, column_create_output_data::ColumnCreateOutputData,
};

pub type ColumnCreateServiceResult<T> = anyhow::Result<T, ColumnCreateServiceError>;

pub trait IColumnCreateService {
    fn handle(
        &self,
        command: ColumnCreateCommand,
    ) -> impl std::future::Future<Output = ColumnCreateServiceResult<ColumnCreateOutputData>> + Send;
}

#[derive(Debug, Error)]
pub enum ColumnCreateServiceError {
    // repository errors
    #[error("ColumnRepositoryError: [{0}]")]
    ColumnRepositoryError(ColumnRepositoryError),

    // value object errors
    #[error("ColumnNameError: [{0}]")]
    ColumnNameError(ColumnNameError),
    #[error("ColumnIdError: [{0}]")]
    ColumnIdError(ColumnIdError),
    #[error("ColumnCellValueError: [{0}]")]
    ColumnCellValueError(ColumnCellValueError),
    #[error("ColumnDirectoryIdError: [{0}]")]
    ColumnDirectoryIdError(ColumnDirectoryIdError),

    // factory errors
    #[error("ColumnFactoryError: [{0}]")]
    ColumnFactoryError(ColumnFactoryError),
}
