use sqlx::{pool::PoolConnection, Sqlite, SqlitePool};
use src_domain::models::column::{column::Column, column_cell::{column_cell::ColumnCell, column_cell_id::ColumnCellId}, column_directory::{column_directory::ColumnDirectory, column_directory_id::ColumnDirectoryId}, column_id::ColumnId, column_repository::{
    ColumnRepositoryError, ColumnRepositoryResult, IColumnRepository,
}};

use super::sqlx_internal_column_repository::InternalColumnRepository;

pub struct SqlxColumnRepository {
    pool: SqlitePool,
}

impl SqlxColumnRepository {
    pub fn new(pool: SqlitePool) -> Self {
        SqlxColumnRepository { pool }
    }

    async fn connection(&self) -> ColumnRepositoryResult<PoolConnection<Sqlite>> {
        self.pool
            .acquire()
            .await
            .map_err(|e| ColumnRepositoryError::Unexpected(Box::new(e)))
    }
}

impl IColumnRepository for SqlxColumnRepository {
    async fn save(&self, column: &Column) -> ColumnRepositoryResult<ColumnId> {
        let mut conn = self.connection().await?;
        let mut internal_column_repository = InternalColumnRepository::new(&mut conn);
        internal_column_repository.save(column).await
    }

    async fn find(&self, id: &ColumnId) -> ColumnRepositoryResult<Option<Column>> {
        todo!()
    }

    async fn find_by_ids(
        &self,
        ids: &Vec<ColumnId>,
    ) -> ColumnRepositoryResult<Vec<Column>> {
        todo!()
    }

    async fn find_by_directory_id(
        &self,
        directory_id: &ColumnDirectoryId,
    ) -> ColumnRepositoryResult<Vec<Column>> {
        todo!()
    }

    async fn find_all(&self) -> ColumnRepositoryResult<Vec<Column>> {
        todo!()
    }

    async fn delete(&self, column: Column) -> ColumnRepositoryResult<()> {
        todo!()
    }

    async fn save_cell(
        &self,
        cell: &ColumnCell,
    ) -> ColumnRepositoryResult<ColumnCellId> {
        todo!()
    }

    async fn find_cell(
        &self,
        id: &ColumnCellId,
    ) -> ColumnRepositoryResult<Option<ColumnCell>> {
        todo!()
    }

    async fn find_cells_by_column_id(
        &self,
        column_id: &ColumnId,
    ) -> ColumnRepositoryResult<Vec<ColumnCell>> {
        todo!()
    }

    async fn find_cells_by_ids(
        &self,
        ids: &Vec<ColumnCellId>,
    ) -> ColumnRepositoryResult<Vec<ColumnCell>> {
        todo!()
    }

    async fn delete_cell(
        &self,
        cell: ColumnCell,
    ) -> ColumnRepositoryResult<()> {
        todo!()
    }

    async fn save_directory(
        &self,
        directory: &ColumnDirectory,
    ) -> ColumnRepositoryResult<ColumnDirectoryId>
    {
        todo!()
    }

    async fn find_directory(
        &self,
        id: &ColumnDirectoryId,
    ) -> ColumnRepositoryResult<
        Option<ColumnDirectory>,
    > {
        todo!()
    }

    async fn find_children_directories(
        &self,
        parent_id: &ColumnDirectoryId,
    ) -> ColumnRepositoryResult<Vec<ColumnDirectory>>
    {
        todo!()
    }

    async fn delete_directory(
        &self,
        directory: ColumnDirectory,
    ) -> ColumnRepositoryResult<()> {
        todo!()
    }
}
