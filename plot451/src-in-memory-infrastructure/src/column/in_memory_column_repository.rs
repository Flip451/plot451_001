use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use src_domain::{
    models::column::{
        column::Column,
        column_cell::{column_cell::ColumnCell, column_cell_id::ColumnCellId},
        column_directory::{
            column_directory::ColumnDirectory, column_directory_id::ColumnDirectoryId,
        },
        column_id::ColumnId,
        column_repository::{ColumnRepositoryError, ColumnRepositoryResult, IColumnRepository},
    },
    shared::value_object::ValueObject,
};

#[derive(Default)]
struct Store {
    current_cell_id: u64,
    cell_store: HashMap<ColumnCellId, ColumnCell>,
    current_column_id: u64,
    column_store: HashMap<ColumnId, Column>,
    current_directory_id: u64,
    directory_store: HashMap<ColumnDirectoryId, ColumnDirectory>,
}

pub struct InMemoryColumnRepository {
    store: Arc<RwLock<Store>>,
}

impl InMemoryColumnRepository {
    pub fn new() -> Self {
        Self {
            store: Arc::default(),
        }
    }

    fn next_column_id(column_store: &mut RwLockWriteGuard<Store>) -> ColumnId {
        column_store.current_column_id += 1;
        ColumnId::new(column_store.current_column_id.to_string()).unwrap()
    }

    fn next_cell_id(column_store: &mut RwLockWriteGuard<Store>) -> ColumnCellId {
        column_store.current_cell_id += 1;
        ColumnCellId::new(column_store.current_cell_id.to_string()).unwrap()
    }

    fn next_directory_id(directory_store: &mut RwLockWriteGuard<Store>) -> ColumnDirectoryId {
        directory_store.current_directory_id += 1;
        ColumnDirectoryId::new(directory_store.current_directory_id.to_string()).unwrap()
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<Store> {
        self.store.write().unwrap()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<Store> {
        self.store.read().unwrap()
    }

    fn delete_directory_and_contents(
        store: &mut RwLockWriteGuard<Store>,
        directory_id: ColumnDirectoryId,
    ) {
        // ディレクトリ内のカラムの一覧
        let column_ids = store
            .column_store
            .iter()
            .filter(|&(_, column)| column.directory_id() == &directory_id)
            .map(|(column_id, _)| column_id.clone())
            .collect::<Vec<ColumnId>>();
        // ディレクトリ内のカラムを削除
        for column_id in column_ids {
            let column = store.column_store.get(&column_id).unwrap().clone();
            // カラムに紐づくセルを削除
            for cell_id in column.cells() {
                store.cell_store.remove(cell_id);
            }
            // カラム自体を削除
            store.column_store.remove(&column_id);
        }
        // ディレクトリ内のディレクトリの一覧
        let directories_in_directory = store
            .directory_store
            .iter()
            .filter(|&(_, directory)| directory.parent().as_ref() == Some(&directory_id))
            .map(|(directory_id, _)| directory_id.clone())
            .collect::<Vec<ColumnDirectoryId>>();
        // ディレクトリ内のディレクトリを削除
        for directory_id in directories_in_directory {
            Self::delete_directory_and_contents(store, directory_id);
        }
        // ディレクトリ自体を削除
        store.directory_store.remove(&directory_id);
    }
}

impl IColumnRepository for InMemoryColumnRepository {
    async fn save(&self, column: &Column) -> ColumnRepositoryResult<ColumnId> {
        let mut column = column.clone();
        let mut store = self.write_store_ref();
        let id = match column.id_wrapped() {
            Some(id) => id.clone(),
            None => {
                let id = Self::next_column_id(&mut store);
                column.set_id(id.clone());
                id
            }
        };
        store.column_store.insert(id.clone(), column);
        Ok(id)
    }

    async fn find(&self, id: &ColumnId) -> ColumnRepositoryResult<Option<Column>> {
        let store = self.read_store_ref();
        Ok(store.column_store.get(id).map(|column| column.clone()))
    }

    async fn find_by_ids(&self, ids: &Vec<ColumnId>) -> ColumnRepositoryResult<Vec<Column>> {
        let store = self.read_store_ref();
        let mut columns = vec![];
        for id in ids {
            if let Some(column) = store.column_store.get(id) {
                columns.push(column.clone());
            } else {
                return Err(ColumnRepositoryError::NotAllColumnsFound(ids.clone()));
            }
        }
        Ok(columns)
    }

    async fn find_all(&self) -> ColumnRepositoryResult<Vec<Column>> {
        let store = self.read_store_ref();
        Ok(store.column_store.values().cloned().collect())
    }

    async fn delete(&self, column: Column) -> ColumnRepositoryResult<()> {
        let mut store = self.write_store_ref();
        for cell_id in column.cells() {
            store.cell_store.remove(cell_id);
        }
        store.column_store.remove(column.id());
        Ok(())
    }

    async fn save_cell(&self, cell: &ColumnCell) -> ColumnRepositoryResult<ColumnCellId> {
        let mut cell = cell.clone();
        let mut store = self.write_store_ref();
        let id = match cell.id_wrapped() {
            Some(id) => id.clone(),
            None => {
                let id = Self::next_cell_id(&mut store);
                cell.set_id(id.clone());
                id
            }
        };
        store.cell_store.insert(id.clone(), cell.clone());
        Ok(id)
    }

    async fn find_cell(&self, id: &ColumnCellId) -> ColumnRepositoryResult<Option<ColumnCell>> {
        let store = self.read_store_ref();
        Ok(store.cell_store.get(id).map(|cell| cell.clone()))
    }

    async fn find_cells_by_column_id(
        &self,
        column_id: &ColumnId,
    ) -> ColumnRepositoryResult<Vec<ColumnCell>> {
        let store = self.read_store_ref();
        let column = store
            .column_store
            .get(column_id)
            .ok_or(ColumnRepositoryError::ColumnNotFound(column_id.clone()))?;
        let cell_ids = column.cells();
        let cells_found = store
            .cell_store
            .iter()
            .filter(|&(id, _)| cell_ids.contains(id))
            .map(|(_, cell)| cell.clone())
            .collect();
        Ok(cells_found)
    }

    async fn find_cells_by_ids(
        &self,
        ids: &Vec<ColumnCellId>,
    ) -> ColumnRepositoryResult<Vec<ColumnCell>> {
        let store = self.read_store_ref();
        let mut cells = vec![];
        for id in ids {
            if let Some(cell) = store.cell_store.get(id) {
                cells.push(cell.clone());
            } else {
                return Err(ColumnRepositoryError::NotAllCellsFound(ids.clone()));
            }
        }
        Ok(cells)
    }

    async fn delete_cell(&self, cell: ColumnCell) -> ColumnRepositoryResult<()> {
        let mut store = self.write_store_ref();
        store.cell_store.remove(cell.id());
        Ok(())
    }

    async fn find_by_directory_id(
        &self,
        directory_id: &ColumnDirectoryId,
    ) -> ColumnRepositoryResult<Vec<Column>> {
        let store = self.read_store_ref();
        let columns_found = store
            .column_store
            .iter()
            .filter(|&(_, column)| column.directory_id() == directory_id)
            .map(|(_, column)| column.clone())
            .collect();
        Ok(columns_found)
    }

    async fn save_directory(
        &self,
        directory: &ColumnDirectory,
    ) -> ColumnRepositoryResult<ColumnDirectoryId> {
        let mut directory = directory.clone();
        let mut store = self.write_store_ref();
        let id = match directory.id_wrapped() {
            Some(id) => id.clone(),
            None => {
                let id = Self::next_directory_id(&mut store);
                directory.set_id(id.clone());
                id
            }
        };
        store.directory_store.insert(id.clone(), directory);
        Ok(id)
    }

    async fn find_directory(
        &self,
        id: &ColumnDirectoryId,
    ) -> ColumnRepositoryResult<Option<ColumnDirectory>> {
        let store = self.read_store_ref();
        Ok(store
            .directory_store
            .get(id)
            .map(|directory| directory.clone()))
    }

    async fn find_children_directories(
        &self,
        parent_id: &ColumnDirectoryId,
    ) -> ColumnRepositoryResult<Vec<ColumnDirectory>> {
        let store = self.read_store_ref();
        let directories_found = store
            .directory_store
            .iter()
            .filter(|&(_, directory)| directory.parent().as_ref() == Some(parent_id))
            .map(|(_, directory)| directory.clone())
            .collect();
        Ok(directories_found)
    }

    async fn delete_directory(&self, directory: ColumnDirectory) -> ColumnRepositoryResult<()> {
        let mut store = self.write_store_ref();
        Self::delete_directory_and_contents(&mut store, directory.id().clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use src_domain::models::column::{
        column_cell::column_cell_value::ColumnCellValue,
        column_directory::column_directory_name::ColumnDirectoryName, column_name::ColumnName,
    };

    use super::*;

    #[tokio::test]
    async fn test_save_with_none_id() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();
        let mut column = Column::new(
            ColumnName::new("column_name1".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![],
        );
        let id = repository.save(&column).await.unwrap();
        column.set_id(id.clone());
        let store = repository.read_store_ref();
        assert_eq!(store.column_store.get(&id).unwrap(), &column);
        Ok(())
    }

    #[tokio::test]
    async fn test_save_with_some_id() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();

        // 保存用のカラムの作成
        let column = Column::reconstruct(
            ColumnId::new("1".to_string())?,
            ColumnName::new("column_name1".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![],
        );

        // save メソッドのテスト
        let id = repository.save(&column).await?;

        // 保存したカラムがストア上に存在することを確認
        let store = repository.read_store_ref();
        assert_eq!(store.column_store.get(&id).unwrap(), &column);
        Ok(())
    }

    #[tokio::test]
    async fn test_find() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();

        // ストア内に保存するデータの作成
        let column_id = ColumnId::new("1".to_string())?;
        let column = Column::reconstruct(
            column_id.clone(),
            ColumnName::new("column_name1".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![],
        );

        // ストアにデータを保存
        {
            let mut store = repository.write_store_ref();
            store.column_store.insert(column_id.clone(), column.clone());
        }

        // find メソッドのテスト
        let found_column = repository.find(&column_id).await?;
        match found_column {
            Some(found_column) => {
                assert_eq!(found_column, column);
            }
            None => {
                panic!("Column not found");
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_ids() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();

        // ストア内に保存するデータの作成
        let column_id1 = ColumnId::new("1".to_string())?;
        let column1 = Column::reconstruct(
            column_id1.clone(),
            ColumnName::new("column_name1".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![],
        );

        let column_id2 = ColumnId::new("2".to_string())?;
        let column2 = Column::reconstruct(
            column_id2.clone(),
            ColumnName::new("column_name2".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![],
        );

        let column_id3 = ColumnId::new("3".to_string())?;
        let column3 = Column::reconstruct(
            column_id3.clone(),
            ColumnName::new("column_name3".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![],
        );

        // ストアにデータを保存
        {
            let mut store = repository.write_store_ref();
            store
                .column_store
                .insert(column_id1.clone(), column1.clone());
            store
                .column_store
                .insert(column_id2.clone(), column2.clone());
            store
                .column_store
                .insert(column_id3.clone(), column3.clone());
        }

        // find_by_ids メソッドのテスト
        let found_columns = repository
            .find_by_ids(&vec![column_id1.clone(), column_id2.clone()])
            .await?;
        assert_eq!(found_columns, vec![column1, column2]);
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_ids_not_found() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();

        // ストア内に保存するデータの作成
        let column_id1 = ColumnId::new("1".to_string())?;
        let column1 = Column::reconstruct(
            column_id1.clone(),
            ColumnName::new("column_name1".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![],
        );

        let column_id2 = ColumnId::new("2".to_string())?;
        let column2 = Column::reconstruct(
            column_id2.clone(),
            ColumnName::new("column_name2".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![],
        );

        let column_id3 = ColumnId::new("3".to_string())?;
        let column3 = Column::reconstruct(
            column_id3.clone(),
            ColumnName::new("column_name3".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![],
        );

        // ストアにデータを保存
        {
            let mut store = repository.write_store_ref();
            store
                .column_store
                .insert(column_id1.clone(), column1.clone());
            store
                .column_store
                .insert(column_id2.clone(), column2.clone());
            store
                .column_store
                .insert(column_id3.clone(), column3.clone());
        }

        // find_by_ids メソッドのテスト
        let found_columns = repository
            .find_by_ids(&vec![
                column_id1.clone(),
                column_id2.clone(),
                ColumnId::new("4".to_string())?,
            ])
            .await;
        if let Err(ColumnRepositoryError::NotAllColumnsFound(ids)) = found_columns {
            assert_eq!(
                ids,
                vec![
                    column_id1.clone(),
                    column_id2.clone(),
                    ColumnId::new("4".to_string())?
                ]
            );
            Ok(())
        } else {
            panic!("Unexpected result: {:?}", found_columns);
        }
    }

    #[tokio::test]
    async fn test_find_all() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();

        // ストア内に保存するデータの作成
        let column_id1 = ColumnId::new("1".to_string())?;
        let column1 = Column::reconstruct(
            column_id1.clone(),
            ColumnName::new("column_name1".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![],
        );

        let column_id2 = ColumnId::new("2".to_string())?;
        let column2 = Column::reconstruct(
            column_id2.clone(),
            ColumnName::new("column_name2".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![],
        );

        let column_id3 = ColumnId::new("3".to_string())?;
        let column3 = Column::reconstruct(
            column_id3.clone(),
            ColumnName::new("column_name3".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![],
        );

        // ストアにデータを保存
        {
            let mut store = repository.write_store_ref();
            store
                .column_store
                .insert(column_id1.clone(), column1.clone());
            store
                .column_store
                .insert(column_id2.clone(), column2.clone());
            store
                .column_store
                .insert(column_id3.clone(), column3.clone());
        }

        // find_all メソッドのテスト
        let found_columns = repository.find_all().await?;
        assert_eq!(
            HashSet::from_iter(found_columns.iter().cloned()),
            HashSet::from([column1, column2, column3])
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_delete() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();

        // ストア内に保存するデータの作成
        // 1. セル
        let cell_id1 = ColumnCellId::new("1".to_string())?;
        let cell1 = ColumnCell::reconstruct(cell_id1.clone(), ColumnCellValue::new(Some(1.0))?);

        let cell_id2 = ColumnCellId::new("2".to_string())?;
        let cell2 = ColumnCell::reconstruct(cell_id2.clone(), ColumnCellValue::new(Some(2.0))?);

        let cell_id3 = ColumnCellId::new("3".to_string())?;
        let cell3 = ColumnCell::reconstruct(cell_id3.clone(), ColumnCellValue::new(Some(3.0))?);

        let cell_id4 = ColumnCellId::new("4".to_string())?;
        let cell4 = ColumnCell::reconstruct(cell_id4.clone(), ColumnCellValue::new(None)?);

        let cell_id5 = ColumnCellId::new("5".to_string())?;
        let cell5 = ColumnCell::reconstruct(cell_id5.clone(), ColumnCellValue::new(Some(5.0))?);

        let cell_id6 = ColumnCellId::new("6".to_string())?;
        let cell6 = ColumnCell::reconstruct(cell_id6.clone(), ColumnCellValue::new(None)?);

        // 2. カラム
        let column_id1 = ColumnId::new("1".to_string())?;
        let column1 = Column::reconstruct(
            column_id1.clone(),
            ColumnName::new("column_name1".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![cell_id1.clone(), cell_id2.clone()],
        );

        let column_id2 = ColumnId::new("2".to_string())?;
        let column2 = Column::reconstruct(
            column_id2.clone(),
            ColumnName::new("column_name2".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![cell_id3.clone(), cell_id4.clone()],
        );

        let column_id3 = ColumnId::new("3".to_string())?;
        let column3 = Column::reconstruct(
            column_id3.clone(),
            ColumnName::new("column_name3".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![cell_id5.clone(), cell_id6.clone()],
        );

        // ストアにデータを保存
        {
            let mut store = repository.write_store_ref();

            // セル
            store.cell_store.insert(cell_id1.clone(), cell1.clone());
            store.cell_store.insert(cell_id2.clone(), cell2.clone());
            store.cell_store.insert(cell_id3.clone(), cell3.clone());
            store.cell_store.insert(cell_id4.clone(), cell4.clone());
            store.cell_store.insert(cell_id5.clone(), cell5.clone());
            store.cell_store.insert(cell_id6.clone(), cell6.clone());

            // カラム
            store
                .column_store
                .insert(column_id1.clone(), column1.clone());
            store
                .column_store
                .insert(column_id2.clone(), column2.clone());
            store
                .column_store
                .insert(column_id3.clone(), column3.clone());
        }

        // delete メソッドのテスト
        repository.delete(column2.clone()).await?;
        let store = repository.read_store_ref();

        // カラムが削除されたことを確認
        assert!(store.column_store.get(&column_id1).is_some());
        assert!(store.column_store.get(&column_id2).is_none()); // ここだけ None
        assert!(store.column_store.get(&column_id3).is_some());

        // セルが削除されたことを確認
        assert!(store.cell_store.get(&cell_id1).is_some());
        assert!(store.cell_store.get(&cell_id2).is_some());
        assert!(store.cell_store.get(&cell_id3).is_none()); // ここだけ None
        assert!(store.cell_store.get(&cell_id4).is_none()); // ここだけ None
        assert!(store.cell_store.get(&cell_id5).is_some());
        assert!(store.cell_store.get(&cell_id6).is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_save_cell_with_none_id() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();
        // 保存用のセルの作成
        let mut cell = ColumnCell::new(ColumnCellValue::new(Some(1.0))?);

        // save_cell メソッドのテスト
        let id = repository.save_cell(&cell).await.unwrap();
        cell.set_id(id.clone());

        // 保存したセルがストア上に存在することを確認
        let store = repository.read_store_ref();
        assert_eq!(store.cell_store.get(&id).unwrap(), &cell);
        Ok(())
    }

    #[tokio::test]
    async fn test_save_cell_with_some_id() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();

        // 保存用のセルの作成
        let cell = ColumnCell::reconstruct(
            ColumnCellId::new("1".to_string())?,
            ColumnCellValue::new(Some(1.0))?,
        );

        // save_cell メソッドのテスト
        let id = repository.save_cell(&cell).await?;

        // 保存したセルがストア上に存在することを確認
        let store = repository.read_store_ref();
        assert_eq!(store.cell_store.get(&id).unwrap(), &cell);
        Ok(())
    }

    #[tokio::test]
    async fn test_save_cell() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();

        // 保存用のセルの作成
        let cell_id = ColumnCellId::new("1".to_string())?;
        let cell = ColumnCell::reconstruct(cell_id.clone(), ColumnCellValue::new(Some(1.0))?);

        // ストアにデータを保存
        {
            let mut store = repository.write_store_ref();
            store.cell_store.insert(cell_id.clone(), cell.clone());
        }

        // 保存したセルがストア上に存在することを確認
        let store = repository.read_store_ref();
        assert_eq!(store.cell_store.get(&cell_id).unwrap(), &cell);
        Ok(())
    }

    #[tokio::test]
    async fn test_find_cells_by_column_id() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();

        // ストア内に保存するデータの作成
        let column_id = ColumnId::new("1".to_string())?;
        let cell_id1 = ColumnCellId::new("1".to_string())?;
        let cell1 = ColumnCell::reconstruct(cell_id1.clone(), ColumnCellValue::new(Some(1.0))?);
        let cell_id2 = ColumnCellId::new("2".to_string())?;
        let cell2 = ColumnCell::reconstruct(cell_id2.clone(), ColumnCellValue::new(Some(2.0))?);
        let cell_id3 = ColumnCellId::new("3".to_string())?;
        let cell3 = ColumnCell::reconstruct(cell_id3.clone(), ColumnCellValue::new(Some(3.0))?);

        // ストアにデータを保存
        {
            let mut store = repository.write_store_ref();
            store.cell_store.insert(cell_id1.clone(), cell1.clone());
            store.cell_store.insert(cell_id2.clone(), cell2.clone());
            store.cell_store.insert(cell_id3.clone(), cell3.clone());
            store.column_store.insert(
                column_id.clone(),
                Column::reconstruct(
                    column_id.clone(),
                    ColumnName::new("column_name1".to_string())?,
                    ColumnDirectoryId::new("0".to_string())?,
                    vec![cell_id1.clone(), cell_id2.clone(), cell_id3.clone()],
                ),
            );
        }

        // find_cells_by_column_id メソッドのテスト
        let found_cells = repository.find_cells_by_column_id(&column_id).await?;
        assert_eq!(found_cells.len(), 3);
        for cell in vec![cell1.clone(), cell2.clone(), cell3.clone()] {
            assert!(found_cells.contains(&cell));
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_find_cells_by_ids() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();

        // ストア内に保存するデータの作成
        let cell_id1 = ColumnCellId::new("1".to_string())?;
        let cell1 = ColumnCell::reconstruct(cell_id1.clone(), ColumnCellValue::new(Some(1.0))?);

        let cell_id2 = ColumnCellId::new("2".to_string())?;
        let cell2 = ColumnCell::reconstruct(cell_id2.clone(), ColumnCellValue::new(Some(2.0))?);

        let cell_id3 = ColumnCellId::new("3".to_string())?;
        let cell3 = ColumnCell::reconstruct(cell_id3.clone(), ColumnCellValue::new(Some(3.0))?);

        // ストアにデータを保存
        {
            let mut store = repository.write_store_ref();
            store.cell_store.insert(cell_id1.clone(), cell1.clone());
            store.cell_store.insert(cell_id2.clone(), cell2.clone());
            store.cell_store.insert(cell_id3.clone(), cell3.clone());
        }

        // find_cells_by_ids メソッドのテスト
        let found_cells = repository
            .find_cells_by_ids(&vec![cell_id1.clone(), cell_id2.clone(), cell_id3.clone()])
            .await?;
        assert_eq!(found_cells, vec![cell1, cell2, cell3],);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_cell() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();

        // ストア内に保存するデータの作成
        let cell_id1 = ColumnCellId::new("1".to_string())?;
        let cell1 = ColumnCell::reconstruct(cell_id1.clone(), ColumnCellValue::new(Some(1.0))?);

        let cell_id2 = ColumnCellId::new("2".to_string())?;
        let cell2 = ColumnCell::reconstruct(cell_id2.clone(), ColumnCellValue::new(Some(2.0))?);

        let cell_id3 = ColumnCellId::new("3".to_string())?;
        let cell3 = ColumnCell::reconstruct(cell_id3.clone(), ColumnCellValue::new(Some(3.0))?);

        // ストアにデータを保存
        {
            let mut store = repository.write_store_ref();
            store.cell_store.insert(cell_id1.clone(), cell1.clone());
            store.cell_store.insert(cell_id2.clone(), cell2.clone());
            store.cell_store.insert(cell_id3.clone(), cell3.clone());
        }

        // delete_cell メソッドのテスト
        repository.delete_cell(cell2.clone()).await?;
        let store = repository.read_store_ref();
        assert!(store.cell_store.get(&cell_id1).is_some());
        assert!(store.cell_store.get(&cell_id2).is_none()); // ここだけ None
        assert!(store.cell_store.get(&cell_id3).is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_directory_id() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();

        // ストア内に保存するデータの作成
        let directory_id = ColumnDirectoryId::new("1".to_string())?;
        let column_id1 = ColumnId::new("1".to_string())?;
        let column1 = Column::reconstruct(
            column_id1.clone(),
            ColumnName::new("column_name1".to_string())?,
            directory_id.clone(),
            vec![],
        );

        let column_id2 = ColumnId::new("2".to_string())?;
        let column2 = Column::reconstruct(
            column_id2.clone(),
            ColumnName::new("column_name2".to_string())?,
            directory_id.clone(),
            vec![],
        );

        let column_id3 = ColumnId::new("3".to_string())?;
        let column3 = Column::reconstruct(
            column_id3.clone(),
            ColumnName::new("column_name3".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![],
        );

        // ストアにデータを保存
        {
            let mut store = repository.write_store_ref();
            store
                .column_store
                .insert(column_id1.clone(), column1.clone());
            store
                .column_store
                .insert(column_id2.clone(), column2.clone());
            store
                .column_store
                .insert(column_id3.clone(), column3.clone());
        }

        // find_by_directory_id メソッドのテスト
        let found_columns = repository.find_by_directory_id(&directory_id).await?;
        assert_eq!(
            HashSet::from_iter(found_columns.iter().cloned()),
            HashSet::from([column1, column2])
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_save_directory_with_none_id() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();

        // 保存用のディレクトリの作成
        let mut directory = ColumnDirectory::new(
            ColumnDirectoryName::new("directory_name1".to_string())?,
            None,
        );

        // save_directory メソッドのテスト
        let id = repository.save_directory(&directory).await?;
        let store = repository.read_store_ref();
        directory.set_id(id.clone());

        assert_eq!(store.directory_store.get(&id).unwrap(), &directory);
        Ok(())
    }

    #[tokio::test]
    async fn test_save_directory_with_some_id() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();

        // 保存用のディレクトリの作成
        let directory = ColumnDirectory::reconstruct(
            ColumnDirectoryId::new("1".to_string())?,
            ColumnDirectoryName::new("directory_name1".to_string())?,
            None,
        );

        // save_directory メソッドのテスト
        let id = repository.save_directory(&directory).await?;
        let store = repository.read_store_ref();

        assert_eq!(store.directory_store.get(&id).unwrap(), &directory);
        Ok(())
    }

    #[tokio::test]
    async fn test_find_directory() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();

        // ストア内に保存するデータの作成
        let directory_id = ColumnDirectoryId::new("1".to_string())?;
        let directory = ColumnDirectory::reconstruct(
            directory_id.clone(),
            ColumnDirectoryName::new("directory_name1".to_string())?,
            None,
        );

        // ストアにデータを保存
        {
            let mut store = repository.write_store_ref();
            store
                .directory_store
                .insert(directory_id.clone(), directory.clone());
        }

        // find メソッドのテスト
        let found_directory = repository.find_directory(&directory_id).await?;
        match found_directory {
            Some(found_directory) => {
                assert_eq!(found_directory, directory);
            }
            None => {
                panic!("directory not found");
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_find_children_directories() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();

        // データ構造
        // root--+--directory1--+--directory2
        //       |              +--directory3
        //       +--directory4

        // ストア内に保存するデータの作成
        let directory_id1 = ColumnDirectoryId::new("1".to_string())?;
        let directory1 = ColumnDirectory::reconstruct(
            directory_id1.clone(),
            ColumnDirectoryName::new("directory_name1".to_string())?,
            None,
        );

        let directory_id2 = ColumnDirectoryId::new("2".to_string())?;
        let directory2 = ColumnDirectory::reconstruct(
            directory_id2.clone(),
            ColumnDirectoryName::new("directory_name2".to_string())?,
            Some(directory_id1.clone()),
        );

        let directory_id3 = ColumnDirectoryId::new("3".to_string())?;
        let directory3 = ColumnDirectory::reconstruct(
            directory_id3.clone(),
            ColumnDirectoryName::new("directory_name3".to_string())?,
            Some(directory_id1.clone()),
        );

        let directory_id4 = ColumnDirectoryId::new("4".to_string())?;
        let directory4 = ColumnDirectory::reconstruct(
            directory_id4.clone(),
            ColumnDirectoryName::new("directory_name4".to_string())?,
            None,
        );

        // ストアにデータを保存
        {
            let mut store = repository.write_store_ref();
            store
                .directory_store
                .insert(directory_id1.clone(), directory1.clone());
            store
                .directory_store
                .insert(directory_id2.clone(), directory2.clone());
            store
                .directory_store
                .insert(directory_id3.clone(), directory3.clone());
            store
                .directory_store
                .insert(directory_id4.clone(), directory4.clone());
        }

        // find_children_directories メソッドのテスト
        let found_directories = repository.find_children_directories(&directory_id1).await?;

        assert_eq!(
            HashSet::from_iter(found_directories.iter().cloned()),
            HashSet::from([directory2, directory3])
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_directory() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();

        // データ構造
        // root--+--directory1--+--directory2--+--column1
        //       |              +--column2
        //       |
        //       +--directory3--+--column3

        // ストア内に保存するデータの作成
        // 1. ディレクトリ
        let directory_id1 = ColumnDirectoryId::new("1".to_string())?;
        let directory1 = ColumnDirectory::reconstruct(
            directory_id1.clone(),
            ColumnDirectoryName::new("directory_name1".to_string())?,
            None,
        );

        let directory_id2 = ColumnDirectoryId::new("2".to_string())?;
        let directory2 = ColumnDirectory::reconstruct(
            directory_id2.clone(),
            ColumnDirectoryName::new("directory_name2".to_string())?,
            Some(directory_id1.clone()),
        );

        let directory_id3 = ColumnDirectoryId::new("3".to_string())?;
        let directory3 = ColumnDirectory::reconstruct(
            directory_id3.clone(),
            ColumnDirectoryName::new("directory_name3".to_string())?,
            None,
        );

        // 2. セル
        let cell_id1 = ColumnCellId::new("1".to_string())?;
        let cell1 = ColumnCell::reconstruct(cell_id1.clone(), ColumnCellValue::new(Some(1.0))?);

        let cell_id2 = ColumnCellId::new("2".to_string())?;
        let cell2 = ColumnCell::reconstruct(cell_id2.clone(), ColumnCellValue::new(Some(2.0))?);

        let cell_id3 = ColumnCellId::new("3".to_string())?;
        let cell3 = ColumnCell::reconstruct(cell_id3.clone(), ColumnCellValue::new(Some(3.0))?);

        let cell_id4 = ColumnCellId::new("4".to_string())?;
        let cell4 = ColumnCell::reconstruct(cell_id4.clone(), ColumnCellValue::new(None)?);

        let cell_id5 = ColumnCellId::new("5".to_string())?;
        let cell5 = ColumnCell::reconstruct(cell_id5.clone(), ColumnCellValue::new(Some(5.0))?);

        let cell_id6 = ColumnCellId::new("6".to_string())?;
        let cell6 = ColumnCell::reconstruct(cell_id6.clone(), ColumnCellValue::new(None)?);

        // 3. カラム
        let column_id1 = ColumnId::new("1".to_string())?;
        let column1 = Column::reconstruct(
            column_id1.clone(),
            ColumnName::new("column_name1".to_string())?,
            directory_id2.clone(),
            vec![cell_id1.clone(), cell_id2.clone()],
        );

        let column_id2 = ColumnId::new("2".to_string())?;
        let column2 = Column::reconstruct(
            column_id2.clone(),
            ColumnName::new("column_name2".to_string())?,
            directory_id1.clone(),
            vec![cell_id3.clone(), cell_id4.clone()],
        );

        let column_id3 = ColumnId::new("3".to_string())?;
        let column3 = Column::reconstruct(
            column_id3.clone(),
            ColumnName::new("column_name3".to_string())?,
            directory_id3.clone(),
            vec![cell_id5.clone(), cell_id6.clone()],
        );

        // ストアにデータを保存
        {
            let mut store = repository.write_store_ref();

            // ディレクトリ
            store
                .directory_store
                .insert(directory_id1.clone(), directory1.clone());
            store
                .directory_store
                .insert(directory_id2.clone(), directory2.clone());
            store
                .directory_store
                .insert(directory_id3.clone(), directory3.clone());

            // セル
            store.cell_store.insert(cell_id1.clone(), cell1.clone());
            store.cell_store.insert(cell_id2.clone(), cell2.clone());
            store.cell_store.insert(cell_id3.clone(), cell3.clone());
            store.cell_store.insert(cell_id4.clone(), cell4.clone());
            store.cell_store.insert(cell_id5.clone(), cell5.clone());
            store.cell_store.insert(cell_id6.clone(), cell6.clone());

            // カラム
            store
                .column_store
                .insert(column_id1.clone(), column1.clone());
            store
                .column_store
                .insert(column_id2.clone(), column2.clone());
            store
                .column_store
                .insert(column_id3.clone(), column3.clone());
        }

        // delete_directory メソッドのテスト
        repository.delete_directory(directory1.clone()).await?;

        // データが削除されたことを確認
        let store = repository.read_store_ref();

        // 1. ディレクトリが削除されていることを確認
        assert!(store.directory_store.get(&directory_id1).is_none());
        assert!(store.directory_store.get(&directory_id2).is_none());
        assert!(store.directory_store.get(&directory_id3).is_some()); // ここだけ Some

        // 2. セルが削除されていることを確認
        assert!(store.cell_store.get(&cell_id1).is_none());
        assert!(store.cell_store.get(&cell_id2).is_none());
        assert!(store.cell_store.get(&cell_id3).is_none());
        assert!(store.cell_store.get(&cell_id4).is_none());
        assert!(store.cell_store.get(&cell_id5).is_some()); // ここだけ Some
        assert!(store.cell_store.get(&cell_id6).is_some()); // ここだけ Some

        // 3. カラムが削除されていることを確認
        assert!(store.column_store.get(&column_id1).is_none());
        assert!(store.column_store.get(&column_id2).is_none());
        assert!(store.column_store.get(&column_id3).is_some()); // ここだけ Some

        Ok(())
    }
}
