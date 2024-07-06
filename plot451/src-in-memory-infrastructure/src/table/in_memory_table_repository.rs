use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use src_domain::{
    models::{
        column::column_id::ColumnId,
        table::{
            table::Table,
            table_id::TableId,
            table_repository::{ITableRepository, TableRepositoryResult},
        },
    },
    shared::value_object::ValueObject,
};

#[derive(Default)]
struct Store {
    current_id: u64,
    table_store: HashMap<TableId, Table>,
}

pub struct InMemoryTableRepository {
    data: Arc<RwLock<Store>>,
}

impl InMemoryTableRepository {
    pub fn new() -> Self {
        Self {
            data: Arc::default(),
        }
    }

    fn next_table_id(store: &mut RwLockWriteGuard<Store>) -> TableId {
        store.current_id += 1;
        TableId::new(store.current_id.to_string()).unwrap()
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<Store> {
        self.data.write().unwrap()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<Store> {
        self.data.read().unwrap()
    }
}

impl ITableRepository for InMemoryTableRepository {
    async fn save(&self, table: &Table) -> TableRepositoryResult<TableId> {
        let mut table = table.clone();
        let mut store = self.write_store_ref();
        let id = match table.id() {
            Some(id) => id.clone(),
            None => {
                let id = Self::next_table_id(&mut store);
                table.set_id(id.clone());
                id
            }
        };
        store.table_store.insert(id.clone(), table);
        Ok(id)
    }

    async fn find(&self, id: &TableId) -> TableRepositoryResult<Option<Table>> {
        let store = self.read_store_ref();
        Ok(store.table_store.get(id).map(|table| table.clone()))
    }

    async fn find_parent_table_by_column_id(
        &self,
        column_id: &ColumnId,
    ) -> TableRepositoryResult<Vec<Table>> {
        let store = self.read_store_ref();
        // 指定されたカラムを含むテーブルを探す
        let found_tables = store
            .table_store
            .iter()
            .filter(|&(_, table)| table.columns().contains(column_id))
            .map(|(_, table)| table.clone())
            .collect::<Vec<Table>>();

        Ok(found_tables)
    }

    async fn find_all(&self) -> TableRepositoryResult<Vec<Table>> {
        let store = self.read_store_ref();
        let tables_found = store.table_store.values().cloned().collect();
        Ok(tables_found)
    }

    async fn delete(&self, table: Table) -> TableRepositoryResult<()> {
        let mut store = self.write_store_ref();
        store.table_store.remove(table.id().as_ref().unwrap());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use src_domain::models::table::table_name::TableName;

    use super::*;

    #[tokio::test]
    async fn test_save_with_some_id() -> anyhow::Result<()> {
        let repository = InMemoryTableRepository::new();

        // 保存用のテーブルインスタンスを作成
        let table = Table::new(
            Some(TableId::new("1".to_string())?),
            TableName::new("test_table".to_string())?,
            vec![
                ColumnId::new("1".to_string())?,
                ColumnId::new("2".to_string())?,
            ],
        )?;

        // save メソッドのテスト
        let table_id = repository.save(&table).await?;

        // リポジトリに保存されていることを確認
        let store = repository.read_store_ref();
        assert_eq!(store.table_store.get(&table_id), Some(&table));

        Ok(())
    }

    #[tokio::test]
    async fn test_save_with_none_id() -> anyhow::Result<()> {
        let repository = InMemoryTableRepository::new();

        // 保存用のテーブルインスタンスを作成
        let mut table = Table::new(
            None,
            TableName::new("test_table".to_string())?,
            vec![
                ColumnId::new("1".to_string())?,
                ColumnId::new("2".to_string())?,
            ],
        )?;

        // save メソッドのテスト
        let table_id = repository.save(&table).await?;
        table.set_id(table_id.clone());

        // リポジトリに保存されていることを確認
        let store = repository.read_store_ref();
        assert_eq!(store.table_store.get(&table_id), Some(&table));

        Ok(())
    }

    #[tokio::test]
    async fn test_find() -> anyhow::Result<()> {
        let repository = InMemoryTableRepository::new();

        // ストア内に保存するデータの作成
        let table_id = TableId::new("1".to_string())?;
        let table = Table::new(
            Some(table_id.clone()),
            TableName::new("test_table".to_string())?,
            vec![
                ColumnId::new("1".to_string())?,
                ColumnId::new("2".to_string())?,
            ],
        )?;

        // ストアにデータを保存
        {
            let mut store = repository.write_store_ref();
            store.table_store.insert(table_id.clone(), table.clone());
        }

        // find メソッドのテスト
        let found_table = repository.find(&table_id).await?;
        match found_table {
            Some(found_table) => assert_eq!(found_table, table),
            None => panic!("Table not found"),
        };

        Ok(())
    }

    #[tokio::test]
    async fn test_find_parent_table_by_column_id() -> anyhow::Result<()> {
        let repository = InMemoryTableRepository::new();

        // ストア内に保存するデータの作成
        let column_id1 = ColumnId::new("1".to_string())?;
        let column_id2 = ColumnId::new("2".to_string())?;
        let column_id3 = ColumnId::new("3".to_string())?;

        let table_id1 = TableId::new("1".to_string())?;
        let table1 = Table::new(
            Some(table_id1.clone()),
            TableName::new("test_table1".to_string())?,
            vec![column_id1.clone(), column_id2.clone()],
        )?;

        let table_id2 = TableId::new("2".to_string())?;
        let table2 = Table::new(
            Some(table_id2.clone()),
            TableName::new("test_table2".to_string())?,
            vec![column_id2.clone(), column_id3.clone()],
        )?;

        // ストアにデータを保存
        {
            let mut store = repository.write_store_ref();
            store.table_store.insert(table_id1.clone(), table1.clone());
            store.table_store.insert(table_id2.clone(), table2.clone());
        }

        // find_parent_table_by_column_id メソッドのテスト
        let found_table = repository
            .find_parent_table_by_column_id(&column_id1)
            .await?;
        assert_eq!(found_table, vec![table1.clone()]);

        let found_table = repository
            .find_parent_table_by_column_id(&column_id2)
            .await?;
        assert_eq!(
            HashSet::from_iter(found_table.iter().cloned()),
            HashSet::from([table1.clone(), table2.clone()])
        );

        let found_table = repository
            .find_parent_table_by_column_id(&column_id3)
            .await?;
        assert_eq!(found_table, vec![table2.clone()]);

        let found_table = repository
            .find_parent_table_by_column_id(&ColumnId::new("4".to_string())?)
            .await?;
        assert_eq!(found_table, vec![]);

        Ok(())
    }

    #[tokio::test]
    async fn test_find_all() -> anyhow::Result<()> {
        let repository = InMemoryTableRepository::new();

        // ストア内に保存するデータの作成
        let table_id1 = TableId::new("1".to_string())?;
        let table1 = Table::new(
            Some(table_id1.clone()),
            TableName::new("test_table1".to_string())?,
            vec![
                ColumnId::new("1".to_string())?,
                ColumnId::new("2".to_string())?,
            ],
        )?;

        let table_id2 = TableId::new("2".to_string())?;
        let table2 = Table::new(
            Some(table_id2.clone()),
            TableName::new("test_table2".to_string())?,
            vec![
                ColumnId::new("3".to_string())?,
                ColumnId::new("4".to_string())?,
            ],
        )?;

        // ストアにデータを保存
        {
            let mut store = repository.write_store_ref();
            store.table_store.insert(table_id1.clone(), table1.clone());
            store.table_store.insert(table_id2.clone(), table2.clone());
        }

        // find_all メソッドのテスト
        let found_tables = repository.find_all().await?;
        assert_eq!(
            HashSet::from_iter(found_tables.iter().cloned()),
            HashSet::from([table1, table2])
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_delete() -> anyhow::Result<()> {
        let repository = InMemoryTableRepository::new();

        // ストア内に保存するデータの作成
        let table_id = TableId::new("1".to_string())?;
        let table = Table::new(
            Some(table_id.clone()),
            TableName::new("test_table".to_string())?,
            vec![
                ColumnId::new("1".to_string())?,
                ColumnId::new("2".to_string())?,
            ],
        )?;

        // ストアにデータを保存
        {
            let mut store = repository.write_store_ref();
            store.table_store.insert(table_id.clone(), table.clone());
        }

        // delete メソッドのテスト
        repository.delete(table.clone()).await?;

        // リポジトリから削除されていることを確認
        let store = repository.read_store_ref();
        assert_eq!(store.table_store.get(&table_id), None);

        Ok(())
    }
}
