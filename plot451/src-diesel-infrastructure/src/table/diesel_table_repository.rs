use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};
use src_domain::models::table::{
    table::Table,
    table_id::TableId,
    table_repository::{ITableRepository, TableRepositoryError, TableRepositoryResult},
};

use crate::db::SqlitePool;

#[derive(Clone)]
pub struct DieselTableRepository {
    pool: SqlitePool,
}

impl DieselTableRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    async fn connection(
        &self,
    ) -> TableRepositoryResult<PooledConnection<ConnectionManager<SqliteConnection>>> {
        self.pool
            .get()
            .map_err(|e| TableRepositoryError::Unexpected(Box::new(e)))
    }
}

impl ITableRepository for DieselTableRepository {
    async fn save(&self, table: &Table) -> TableRepositoryResult<TableId> {
        Ok(todo!())
    }

    async fn find(&self, id: &TableId) -> TableRepositoryResult<Option<Table>> {
        todo!()
    }

    async fn find_parent_table_by_column_id(
        &self,
        column_id: &src_domain::models::column::column_id::ColumnId,
    ) -> TableRepositoryResult<Vec<Table>> {
        Ok(todo!())
    }

    async fn find_all(&self) -> TableRepositoryResult<Vec<Table>> {
        Ok(todo!())
    }

    async fn delete(
        &self,
        table: src_domain::models::table::table::Table,
    ) -> TableRepositoryResult<()> {
        Ok(todo!())
    }
}
