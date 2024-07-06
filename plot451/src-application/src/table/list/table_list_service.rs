use thiserror::Error;

use src_domain::models::{
    column::{
        column_id::ColumnIdError, column_name::ColumnNameError,
        column_repository::ColumnRepositoryError,
    },
    table::{table_name::TableNameError, table_repository::TableRepositoryError},
};

use super::{table_list_command::TableListCommand, table_list_output_data::TableListOutputData};

pub type TableListServiceResult<T> = anyhow::Result<T, TableListServiceError>;

pub trait ITableListService {
    async fn handle(
        &self,
        command: TableListCommand,
    ) -> TableListServiceResult<TableListOutputData>;
}

#[derive(Debug, Error)]
pub enum TableListServiceError {
    // repository errors
    #[error("TableRepositoryError: [{0}]")]
    TableRepositoryError(TableRepositoryError),
    #[error("ColumnRepositoryError: [{0}]")]
    ColumnRepositoryError(ColumnRepositoryError),

    // value object errors
    #[error("TableNameError: [{0}]")]
    TableNameError(TableNameError),
    #[error("ColumnIdError: [{0}]")]
    ColumnIdError(ColumnIdError),
    #[error("ColumnIdError: [{0}]")]
    ColumnNameError(ColumnNameError),
}
