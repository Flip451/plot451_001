use sqlx::{FromRow, Row, SqliteConnection};
use src_domain::{
    models::{
        column::column_id::ColumnId,
        table::{
            table::Table,
            table_id::TableId,
            table_name::TableName,
            table_repository::{TableRepositoryError, TableRepositoryResult},
        },
    },
    shared::value_object::ValueObject,
};

pub(super) struct InternalTableRepository<'a> {
    conn: &'a mut SqliteConnection,
}

struct TableWithColumns {
    table_id: i32,
    table_name: String,
    columns: Vec<i32>,
}

impl TableWithColumns {
    fn into_table(self) -> TableRepositoryResult<Table> {
        let table_id = TableId::new(self.table_id.to_string())
            .map_err(|e| TableRepositoryError::TableIdError(e))?;
        let table_name =
            TableName::new(self.table_name).map_err(|e| TableRepositoryError::TableNameError(e))?;
        let columns = self
            .columns
            .into_iter()
            .map(|c| {
                ColumnId::new(c.to_string()).map_err(|e| TableRepositoryError::ColumnIdError(e))
            })
            .collect::<Result<Vec<ColumnId>, TableRepositoryError>>()?;
        let table = Table::reconstruct(table_id, table_name, columns)
            .map_err(|e| TableRepositoryError::TableEntityError(e))?;
        Ok(table)
    }
}

#[derive(FromRow)]
struct TableRow {
    id: i32,
    name: String,
}

#[derive(FromRow)]
struct TableRowWithColumnId {
    id: i32,
    name: String,
    column_id: Option<i32>,
}

// new type pattern
struct TableRowsWithColumnId<'a>(&'a [TableRowWithColumnId]);

impl<'a> TableRowsWithColumnId<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn into_table(self) -> TableRepositoryResult<Table> {
        let Self(rows) = self;
        assert!(rows.len() > 0);

        let mut table_with_columns = TableWithColumns {
            table_id: rows[0].id,
            table_name: rows[0].name.clone(),
            columns: vec![],
        };

        for row in rows.iter() {
            assert_eq!(row.id, rows[0].id);
            assert_eq!(row.name, rows[0].name);
            if let Some(column_id) = row.column_id {
                table_with_columns.columns.push(column_id);
            }
        }

        table_with_columns.into_table()
    }

    fn into_tables(self) -> TableRepositoryResult<Vec<Table>> {
        // このメソッドは、TableRowWithColumnId のスライス内に含まれる行が
        // table_idとsort_orderでソートされていることを前提としている
        let Self(rows) = self;
        match rows.get(0) {
            // テーブルがない場合は空のリストを返す
            None => Ok(vec![]),
            // テーブルがある場合はテーブルごとに分割して返す
            Some(row) => {
                let mut tables = vec![];
                let mut current_table_id = row.id;
                let mut last_index = 0;
                for (index, row) in rows.iter().enumerate() {
                    if current_table_id != row.id {
                        tables.push(TableRowsWithColumnId(&rows[last_index..index]).into_table()?);
                        last_index = index;
                    }
                    current_table_id = row.id;
                }
                tables.push(TableRowsWithColumnId(&rows[last_index..]).into_table()?);
                Ok(tables)
            }
        }
    }
}

impl<'a> InternalTableRepository<'a> {
    pub(super) fn new(conn: &'a mut SqliteConnection) -> Self {
        InternalTableRepository { conn }
    }

    pub(super) async fn save_table(&mut self, table: &Table) -> TableRepositoryResult<TableId> {
        let table_id = match table.id_wrapped() {
            // テーブル ID がある場合は UPDATE
            Some(table_id) => {
                let sql = "UPDATE tables SET name = $2 WHERE id = $1";

                sqlx::query(sql)
                    .bind(table_id.value())
                    .bind(table.name().value())
                    .execute(&mut *self.conn)
                    .await
                    .map_err(|e| TableRepositoryError::Unexpected(Box::new(e)))?;

                table_id.clone()
            }
            // テーブル ID がない場合は INSERT
            None => {
                let sql = "INSERT INTO tables (name) VALUES ($1) RETURNING id";

                let row = sqlx::query(sql)
                    .bind(table.name().value())
                    .fetch_one(&mut *self.conn)
                    .await
                    .map_err(|e| TableRepositoryError::Unexpected(Box::new(e)))?;

                let table_id = TableId::new(
                    row.try_get::<i64, &'static str>("id")
                        .map_err(|e| TableRepositoryError::Unexpected(Box::new(e)))?
                        .to_string(),
                )
                .map_err(|e| TableRepositoryError::TableIdError(e))?;

                table_id
            }
        };

        let sql = "DELETE FROM table_columns WHERE table_id = $1";

        sqlx::query(sql)
            .bind(table_id.value())
            .execute(&mut *self.conn)
            .await
            .map_err(|e| TableRepositoryError::Unexpected(Box::new(e)))?;

        let sql = r#"
INSERT INTO table_columns (table_id, column_id, sort_order)
VALUES ($1, $2, $3)
"#;

        for (sort_order, column_id) in table.columns().iter().enumerate() {
            sqlx::query(sql)
                .bind(table_id.value())
                .bind(column_id.value())
                .bind(sort_order as i32)
                .execute(&mut *self.conn)
                .await
                .map_err(|e| TableRepositoryError::Unexpected(Box::new(e)))?;
        }

        Ok(table_id)
    }

    pub(super) async fn find_table(
        &mut self,
        table_id: &TableId,
    ) -> TableRepositoryResult<Option<Table>> {
        let sql = r#"
SELECT * FROM tables t
LEFT JOIN table_columns tc ON t.id = tc.table_id
WHERE t.id = $1
ORDER BY tc.sort_order
"#;
        let rows = sqlx::query_as::<_, TableRowWithColumnId>(sql)
            .bind(table_id.value())
            .fetch_all(&mut *self.conn)
            .await
            .map_err(|e| TableRepositoryError::Unexpected(Box::new(e)))?;
        let rows = TableRowsWithColumnId(&rows[..]);
        match rows.len() {
            0 => Ok(None),
            _ => Ok(Some(rows.into_table()?)),
        }
    }

    pub(super) async fn find_parent_table_by_column_id(
        &mut self,
        column_id: &ColumnId,
    ) -> TableRepositoryResult<Vec<Table>> {
        let sql = r#"
SELECT * FROM tables t LEFT JOIN table_columns tc ON t.id = tc.table_id
WHERE t.id IN (
    SELECT table_id FROM table_columns WHERE column_id = $1
)
ORDER BY t.id, tc.sort_order
"#;
        let rows = sqlx::query_as::<_, TableRowWithColumnId>(sql)
            .bind(column_id.value())
            .fetch_all(&mut *self.conn)
            .await
            .map_err(|e| TableRepositoryError::Unexpected(Box::new(e)))?;

        let rows = TableRowsWithColumnId(&rows[..]);

        rows.into_tables()
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
    use std::collections::HashSet;

    use src_domain::models::{
        column::column_directory::column_directory_id::ColumnDirectoryId,
        table::{table::TableEntityError, table_name::TableName},
    };

    use super::*;
    use crate::db;

    #[derive(FromRow)]
    struct DirectoryRow {
        id: i64,
        name: String,
    }

    #[derive(FromRow)]
    struct ColumnRow {
        id: i64,
        name: String,
    }

    #[derive(FromRow)]
    struct TableColumnsRow {
        table_id: i64,
        column_id: i64,
    }

    #[tokio::test]
    async fn test_save_new_table() -> anyhow::Result<()> {
        let pool = db::create_and_migrate_test_pool().await?;
        let mut tx = pool.begin().await?;
        // ディレクトリをDBに登録
        let directory_id = sqlx::query("INSERT INTO directories (name) VALUES ($1) RETURNING id")
            .bind("test_directory")
            .fetch_one(&mut *tx)
            .await?
            .try_get::<i64, &str>("id")?
            .to_string();
        let directory_id = ColumnDirectoryId::new(directory_id)?;

        // カラムをDBに登録
        let column_id1 =
            sqlx::query("INSERT INTO columns (name, directory_id) VALUES ($1, $2) RETURNING id")
                .bind("column_name1")
                .bind(directory_id.value())
                .fetch_one(&mut *tx)
                .await?
                .try_get::<i64, &str>("id")?
                .to_string();
        let column_id1 = ColumnId::new(column_id1)?;

        let mut repo = InternalTableRepository::new(&mut tx);

        // 新しいテーブルの作成をテスト
        let table = Table::new(
            TableName::new("table_name".to_string())?,
            vec![column_id1.clone()],
        )?;
        let table_id = repo.save_table(&table).await?;

        // テーブル ID が空でないことを確認
        assert!(!table_id.value().is_empty());

        // テーブルが保存されていることを確認
        let tables_with_column_id = sqlx::query_as::<_, TableRowWithColumnId>("SELECT * FROM tables t LEFT JOIN table_columns tc ON t.id = tc.table_id ORDER BY tc.sort_order")
            .fetch_all(&mut *tx)
            .await?;
        assert_eq!(tables_with_column_id.len(), 1);
        assert_eq!(tables_with_column_id[0].name, "table_name");
        assert_eq!(
            tables_with_column_id[0].column_id.map(|c| c.to_string()),
            Some(column_id1.clone_value())
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_update_table() -> anyhow::Result<()> {
        let pool = db::create_and_migrate_test_pool().await?;
        let mut tx = pool.begin().await?;

        // ディレクトリをDBに登録
        let directory_id = sqlx::query("INSERT INTO directories (name) VALUES ($1) RETURNING id")
            .bind("test_directory")
            .fetch_one(&mut *tx)
            .await?
            .try_get::<i64, &str>("id")?
            .to_string();
        let directory_id = ColumnDirectoryId::new(directory_id)?;

        // カラムをDBに登録
        let column_id1 =
            sqlx::query("INSERT INTO columns (name, directory_id) VALUES ($1, $2) RETURNING id")
                .bind("column_name1")
                .bind(directory_id.value())
                .fetch_one(&mut *tx)
                .await?
                .try_get::<i64, &str>("id")?
                .to_string();
        let column_id1 = ColumnId::new(column_id1)?;

        let column_id2 =
            sqlx::query("INSERT INTO columns (name, directory_id) VALUES ($1, $2) RETURNING id")
                .bind("column_name2")
                .bind(directory_id.value())
                .fetch_one(&mut *tx)
                .await?
                .try_get::<i64, &str>("id")?
                .to_string();
        let column_id2 = ColumnId::new(column_id2)?;

        // テーブルをDBに登録
        let mut table = Table::new(
            TableName::new("table_name".to_string())?,
            vec![column_id1, column_id2],
        )?;

        let table_id1 = sqlx::query("INSERT INTO tables (name) VALUES ($1) RETURNING id")
            .bind(table.name().value())
            .fetch_one(&mut *tx)
            .await?
            .try_get::<i64, &str>("id")?
            .to_string();
        let table_id1 = TableId::new(table_id1)?;
        table.set_id(table_id1.clone());

        // テーブルが保存されていることを確認
        let tables = sqlx::query_as::<_, TableRow>("SELECT * FROM tables ORDER BY id")
            .fetch_all(&mut *tx)
            .await?;
        assert_eq!(tables[0].name, "table_name");

        // テーブルカラムをDBに登録
        for (sort_order, column_id) in table.columns().iter().enumerate() {
            sqlx::query(
                "INSERT INTO table_columns (table_id, column_id, sort_order) VALUES ($1, $2, $3)",
            )
            .bind(table_id1.value())
            .bind(column_id.value())
            .bind(sort_order as i32)
            .execute(&mut *tx)
            .await?;
        }

        let mut repo = InternalTableRepository::new(&mut tx);

        // テーブル名を変更
        table.change_name(TableName::new("new_table_name".to_string())?);

        // カラムを削除
        let column_id1 = table.columns()[0].clone();
        let column_id2 = table.columns()[1].clone();
        table.remove_column(&column_id2);

        // テーブルを保存
        let table_id2 = repo.save_table(&table).await?;

        // テーブル ID が変わっていないことを確認
        assert_eq!(table_id1, table_id2);

        // テーブルが保存されていることを確認
        let table_rows_with_column_id = sqlx::query_as::<_, TableRowWithColumnId>(
            "SELECT * FROM tables t LEFT JOIN table_columns tc ON t.id = tc.table_id ORDER BY id",
        )
        .fetch_all(&mut *tx)
        .await?;
        assert_eq!(table_rows_with_column_id[0].name, "new_table_name");
        assert_eq!(table_rows_with_column_id.len(), 1);
        assert_eq!(
            table_rows_with_column_id[0]
                .column_id
                .map(|c| c.to_string()),
            Some(column_id1.clone_value())
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_find_table_without_columns() -> anyhow::Result<()> {
        let pool = db::create_and_migrate_test_pool().await?;
        let mut tx = pool.begin().await?;

        // テーブルをDBに登録
        let table_id = sqlx::query("INSERT INTO tables (name) VALUES ($1) RETURNING id")
            .bind("table_name")
            .fetch_one(&mut *tx)
            .await
            .unwrap()
            .try_get::<i64, &str>("id")?
            .to_string();
        let table_id = TableId::new(table_id)?;

        // テーブルが保存されていることを確認
        let tables = sqlx::query_as::<_, TableRow>("SELECT * FROM tables ORDER BY id")
            .fetch_all(&mut *tx)
            .await?;
        assert_eq!(tables[0].name, "table_name");

        let mut repo = InternalTableRepository::new(&mut tx);

        // テーブルを取得
        let result = repo.find_table(&table_id).await;

        if let Err(TableRepositoryError::TableEntityError(TableEntityError::EmptyColumnList)) =
            result
        {
            Ok(())
        } else {
            panic!("unexpected result: {:?}", result);
        }
    }

    #[tokio::test]
    async fn test_find_table_with_some_columns() -> anyhow::Result<()> {
        let pool = db::create_and_migrate_test_pool().await?;
        let mut tx = pool.begin().await?;

        // ディレクトリをDBに登録
        let directory_id = sqlx::query("INSERT INTO directories (name) VALUES ($1) RETURNING id")
            .bind("test_directory")
            .fetch_one(&mut *tx)
            .await?
            .try_get::<i64, &str>("id")?
            .to_string();
        let directory_id = ColumnDirectoryId::new(directory_id)?;

        // カラムをDBに登録
        let column_id1 =
            sqlx::query("INSERT INTO columns (name, directory_id) VALUES ($1, $2) RETURNING id")
                .bind("column1")
                .bind(directory_id.value())
                .fetch_one(&mut *tx)
                .await?
                .try_get::<i64, &str>("id")?
                .to_string();
        let column_id1 = ColumnId::new(column_id1)?;

        let column_id2 =
            sqlx::query("INSERT INTO columns (name, directory_id) VALUES ($1, $2) RETURNING id")
                .bind("column2")
                .bind(directory_id.value())
                .fetch_one(&mut *tx)
                .await?
                .try_get::<i64, &str>("id")?
                .to_string();
        let column_id2 = ColumnId::new(column_id2)?;

        // テーブルをDBに登録
        let table_id = sqlx::query("INSERT INTO tables (name) VALUES ($1) RETURNING id")
            .bind("table_name")
            .fetch_one(&mut *tx)
            .await?
            .try_get::<i64, &str>("id")?
            .to_string();
        let table_id = TableId::new(table_id)?;

        // テーブルが保存されていることを確認
        let tables = sqlx::query_as::<_, TableRow>("SELECT * FROM tables ORDER BY id")
            .fetch_all(&mut *tx)
            .await?;
        assert_eq!(tables[0].name, "table_name");

        // テーブルカラムをDBに登録
        sqlx::query(
            "INSERT INTO table_columns (table_id, column_id, sort_order) VALUES ($1, $2, $3)",
        )
        .bind(table_id.value())
        .bind(column_id1.value())
        .bind(0)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            "INSERT INTO table_columns (table_id, column_id, sort_order) VALUES ($1, $2, $3)",
        )
        .bind(table_id.value())
        .bind(column_id2.value())
        .bind(1)
        .execute(&mut *tx)
        .await?;

        let mut repo = InternalTableRepository::new(&mut tx);

        // テーブルを取得
        let table = repo.find_table(&table_id).await?;

        assert!(table.is_some());
        let table = table.unwrap();

        assert_eq!(table.name().value(), "table_name");
        assert_eq!(table.columns().len(), 2);
        assert_eq!(
            HashSet::from_iter(table.columns().iter()),
            HashSet::from([&column_id1, &column_id2])
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_find_parent_table_by_column_id() -> anyhow::Result<()> {
        let pool = db::create_and_migrate_test_pool().await?;
        let mut tx = pool.begin().await?;

        // ディレクトリをDBに登録
        let directory_id = sqlx::query("INSERT INTO directories (name) VALUES ($1) RETURNING id")
            .bind("test_directory")
            .fetch_one(&mut *tx)
            .await?
            .try_get::<i64, &str>("id")?
            .to_string();
        let directory_id = ColumnDirectoryId::new(directory_id)?;

        // カラムをDBに登録
        let column_id1 =
            sqlx::query("INSERT INTO columns (name, directory_id) VALUES ($1, $2) RETURNING id")
                .bind("column1")
                .bind(directory_id.value())
                .fetch_one(&mut *tx)
                .await?
                .try_get::<i64, &str>("id")?
                .to_string();
        let column_id1 = ColumnId::new(column_id1)?;

        let column_id2 =
            sqlx::query("INSERT INTO columns (name, directory_id) VALUES ($1, $2) RETURNING id")
                .bind("column2")
                .bind(directory_id.value())
                .fetch_one(&mut *tx)
                .await?
                .try_get::<i64, &str>("id")?
                .to_string();
        let column_id2 = ColumnId::new(column_id2)?;

        // テーブルをDBに登録
        let table_id1 = sqlx::query("INSERT INTO tables (name) VALUES ($1) RETURNING id")
            .bind("table_name1")
            .fetch_one(&mut *tx)
            .await?
            .try_get::<i64, &str>("id")?
            .to_string();
        let table_id1 = TableId::new(table_id1)?;

        let table_id2 = sqlx::query("INSERT INTO tables (name) VALUES ($1) RETURNING id")
            .bind("table_name2")
            .fetch_one(&mut *tx)
            .await?
            .try_get::<i64, &str>("id")?
            .to_string();
        let table_id2 = TableId::new(table_id2)?;

        // テーブルが保存されていることを確認
        let tables = sqlx::query_as::<_, TableRow>("SELECT * FROM tables ORDER BY id")
            .fetch_all(&mut *tx)
            .await?;
        assert_eq!(tables.len(), 2);
        assert_eq!(tables[0].name, "table_name1");
        assert_eq!(tables[1].name, "table_name2");

        // テーブルカラムをDBに登録
        sqlx::query(
            "INSERT INTO table_columns (table_id, column_id, sort_order) VALUES ($1, $2, $3)",
        )
        .bind(table_id1.value())
        .bind(column_id1.value())
        .bind(0)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            "INSERT INTO table_columns (table_id, column_id, sort_order) VALUES ($1, $2, $3)",
        )
        .bind(table_id1.value())
        .bind(column_id2.value())
        .bind(1)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            "INSERT INTO table_columns (table_id, column_id, sort_order) VALUES ($1, $2, $3)",
        )
        .bind(table_id2.value())
        .bind(column_id1.value())
        .bind(0)
        .execute(&mut *tx)
        .await?;

        let mut repo = InternalTableRepository::new(&mut tx);

        // カラム1 を子に持つテーブルを取得
        let tables = repo.find_parent_table_by_column_id(&column_id1).await?;

        // 取得されるテーブルは２つ
        assert_eq!(tables.len(), 2);

        // 一つ目のテーブルの確認
        assert_eq!(tables[0].name().value(), "table_name1");
        assert_eq!(tables[0].columns().len(), 2);
        assert_eq!(tables[0].columns()[0], column_id1);
        assert_eq!(tables[0].columns()[1], column_id2);

        // 二つの目のテーブルの確認
        assert_eq!(tables[1].name().value(), "table_name2");
        assert_eq!(tables[1].columns().len(), 1);
        assert_eq!(tables[1].columns()[0], column_id1);

        Ok(())
    }
}
