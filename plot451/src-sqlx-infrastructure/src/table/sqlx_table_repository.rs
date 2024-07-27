use sqlx::{pool::PoolConnection, Sqlite, SqlitePool};
use src_domain::models::{
    column::column_id::ColumnId,
    table::{
        table::Table,
        table_id::TableId,
        table_repository::{ITableRepository, TableRepositoryError, TableRepositoryResult},
    },
};

use super::sqlx_internal_table_repository::InternalTableRepository;

pub struct SqlxTableRepository {
    pool: SqlitePool,
}

impl SqlxTableRepository {
    pub fn new(pool: SqlitePool) -> Self {
        SqlxTableRepository { pool }
    }

    async fn connection(&self) -> TableRepositoryResult<PoolConnection<Sqlite>> {
        self.pool
            .acquire()
            .await
            .map_err(|e| TableRepositoryError::Unexpected(Box::new(e)))
    }
}

impl ITableRepository for SqlxTableRepository {
    async fn save(&self, table: &Table) -> TableRepositoryResult<TableId> {
        let mut conn = self.connection().await?;
        let mut internal_table_repository = InternalTableRepository::new(&mut conn);
        internal_table_repository.save_table(table).await
    }

    async fn find(&self, id: &TableId) -> TableRepositoryResult<Option<Table>> {
        let mut conn = self.connection().await?;
        let mut internal_table_repository = InternalTableRepository::new(&mut conn);
        internal_table_repository.find_table(id).await
    }

    async fn find_parent_table_by_column_id(
        &self,
        column_id: &ColumnId,
    ) -> TableRepositoryResult<Vec<Table>> {
        let mut conn = self.connection().await?;
        let mut internal_table_repository = InternalTableRepository::new(&mut conn);
        internal_table_repository
            .find_parent_table_by_column_id(column_id)
            .await
    }

    async fn find_all(&self) -> TableRepositoryResult<Vec<Table>> {
        let mut conn = self.connection().await?;
        let mut internal_table_repository = InternalTableRepository::new(&mut conn);
        internal_table_repository.find_all().await
    }

    async fn delete(&self, table: Table) -> TableRepositoryResult<()> {
        let mut conn = self.connection().await?;
        let mut internal_table_repository = InternalTableRepository::new(&mut conn);
        internal_table_repository.delete(table).await
    }
}
