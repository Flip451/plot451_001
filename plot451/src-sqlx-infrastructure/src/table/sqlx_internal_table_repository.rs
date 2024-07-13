use sqlx::{FromRow, Row, SqliteConnection};
use src_domain::{
    models::{
        column::column_id::ColumnId,
        table::{
            table::Table,
            table_id::TableId,
            table_repository::{TableRepositoryError, TableRepositoryResult},
        },
    },
    shared::value_object::ValueObject,
};

pub(super) struct InternalTableRepository<'a> {
    conn: &'a mut SqliteConnection,
}

#[derive(FromRow)]
struct TableRow {
    id: i64,
    name: String,
}

impl<'a> InternalTableRepository<'a> {
    pub(super) fn new(conn: &'a mut SqliteConnection) -> Self {
        InternalTableRepository { conn }
    }

    pub(super) async fn save_table(&mut self, table: &Table) -> TableRepositoryResult<TableId> {
        let row = match table.id_wrapped() {
            Some(_) => {
                let sql = r#"
INSERT INTO tables (id, name)
VALUES (&1, $2)
ON CONFLICT (id)
DO UPDATE SET name = $2
RETURNING id
"#;
                sqlx::query(sql)
                    .bind(table.id().value())
                    .bind(table.name().value())
                    .fetch_one(&mut *self.conn)
                    .await
                    .map_err(|e| TableRepositoryError::Unexpected(Box::new(e)))?
            }
            None => {
                let sql = r#"
INSERT INTO tables (name)
VALUES ($1)
RETURNING id
"#;
                sqlx::query(sql)
                    .bind(table.name().value())
                    .fetch_one(&mut *self.conn)
                    .await
                    .map_err(|e| TableRepositoryError::Unexpected(Box::new(e)))?
            }
        };
        let table_id = TableId::new(
            row.try_get::<i64, &'static str>("id")
                .map_err(|e| TableRepositoryError::Unexpected(Box::new(e)))?
                .to_string(),
        )
        .map_err(|e| TableRepositoryError::TableIdError(e))?;
        Ok(table_id)
    }

    pub(super) async fn find(&mut self) -> TableRepositoryResult<Option<Table>> {
        todo!()
    }

    pub(super) async fn find_parent_table_by_column_id(
        &mut self,
        column_id: &ColumnId,
    ) -> TableRepositoryResult<Option<Table>> {
        todo!()
    }

    pub(super) async fn find_all(&mut self) -> TableRepositoryResult<Vec<Table>> {
        todo!()
    }

    pub(super) async fn delete(&mut self) -> TableRepositoryResult<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use src_domain::models::table::{table_factory::ITableFactory, table_name::TableName};

    use super::*;
    use crate::{db, table::sqlx_table_factory::SqlxTableFactory};

    #[tokio::test]
    async fn test_save_new_table() -> anyhow::Result<()> {
        let pool = db::create_and_migrate_test_pool().await?;
        let mut tx = pool.begin().await?;
        let mut repo = InternalTableRepository::new(&mut tx);

        // 新しいテーブルの作成をテスト
        let factrory = SqlxTableFactory::new();
        let table = factrory
            .create_table(
                TableName::new("table_name".to_string())?,
                vec![ColumnId::new("column_id1".to_string())?],
            )
            .await?;

        let table_id = repo.save_table(&table).await?;

        // テーブル ID が空でないことを確認
        assert!(!table_id.value().is_empty());

        // テーブルが保存されていることを確認
        let tables = sqlx::query_as::<_, TableRow>("SELECT * FROM tables ORDER BY id")
            .fetch_all(&mut *tx)
            .await?;
        assert_eq!(tables[0].name, "table_name");
        Ok(())
    }

    async fn test_update_table() -> anyhow::Result<()> {
        let pool = db::create_and_migrate_test_pool().await?;
        let mut tx = pool.begin().await?;
        let mut repo = InternalTableRepository::new(&mut tx);
        let factrory = SqlxTableFactory::new();
        let table = factrory
            .create_table(
                TableName::new("table_name".to_string())?,
                vec![ColumnId::new("column_id1".to_string())?],
            )
            .await?;

        let table_id = repo.save_table(&table).await?;

        assert_eq!(table_id, TableId::new("1".to_string())?);

        let table = factrory
            .create_table(
                TableName::new("table_name".to_string())?,
                vec![ColumnId::new("column_id2".to_string())?],
            )
            .await?;

        let table_id = repo.save_table(&table).await?;

        assert_eq!(table_id, TableId::new("1".to_string())?);
        Ok(())
    }
}
