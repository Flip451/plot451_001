use thiserror::Error;

use src_domain::models::{
    column::{column_id::ColumnIdError, column_repository::ColumnRepositoryError},
    table::{
        no_duplicated_column_names_specification::NoDuplicateColumnNameSpecificationError,
        table_factory::TableFactoryError, table_name::TableNameError,
        table_repository::TableRepositoryError,
    },
};

use super::{
    table_create_command::TableCreateCommand, table_create_output_data::TableCreateOutputData,
};

pub type TableCreateServiceResult<T> = anyhow::Result<T, TableCreateServiceError>;

pub trait ITableCreateService {
    async fn handle(
        &self,
        command: TableCreateCommand,
    ) -> TableCreateServiceResult<TableCreateOutputData>;
}

#[derive(Debug, Error)]
pub enum TableCreateServiceError {
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

    // specification errors
    #[error("DuplicatedColumnNameError: [{0}]")]
    DuplicatedColumnNameError(NoDuplicateColumnNameSpecificationError),

    // factory errors
    #[error("TableFactoryError: [{0}]")]
    TableFactoryError(TableFactoryError),
}
