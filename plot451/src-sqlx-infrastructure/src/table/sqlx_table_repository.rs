use sqlx::{pool::PoolConnection, Sqlite, SqlitePool};
use src_domain::models::{
    column::column_id::ColumnId,
    table::{
        table::Table,
        table_id::TableId,
        table_repository::{ITableRepository, TableRepositoryError, TableRepositoryResult},
    },
};

struct SqlxTableRepository {
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
        todo!()
    }

    async fn find(&self, id: &TableId) -> TableRepositoryResult<Option<Table>> {
        todo!()
    }

    async fn find_parent_table_by_column_id(
        &self,
        column_id: &ColumnId,
    ) -> TableRepositoryResult<Vec<Table>> {
        todo!()
    }

    async fn find_all(&self) -> TableRepositoryResult<Vec<Table>> {
        todo!()
    }

    async fn delete(&self, table: Table) -> TableRepositoryResult<()> {
        todo!()
    }
}
