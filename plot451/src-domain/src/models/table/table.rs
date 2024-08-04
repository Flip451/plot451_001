use thiserror::Error;

use super::table_id::TableId;
use super::table_name::TableName;
use crate::{models::column::column_id::ColumnId, shared::entity::Entity};

// entity
#[derive(Debug, Clone, Eq, Hash)]
pub struct Table {
    id: Option<TableId>,
    name: TableName,
    columns: Vec<ColumnId>,
}

impl Table {
    pub fn new(name: TableName, columns: Vec<ColumnId>) -> Result<Self, TableEntityError> {
        if columns.is_empty() {
            return Err(TableEntityError::EmptyColumnList);
        }
        Ok(Self {
            id: None,
            name,
            columns,
        })
    }

    // Table の再構築
    pub fn reconstruct(
        id: TableId,
        name: TableName,
        columns: Vec<ColumnId>,
    ) -> Result<Self, TableEntityError> {
        if columns.is_empty() {
            return Err(TableEntityError::EmptyColumnList);
        }
        Ok(Self {
            id: Some(id),
            name,
            columns,
        })
    }

    // getter & setter
    pub fn id(&self) -> &TableId {
        &self.id.as_ref().expect("id is not set")
    }

    pub fn id_wrapped(&self) -> &Option<TableId> {
        &self.id
    }

    pub fn set_id(&mut self, id: TableId) {
        if let Some(_) = self.id {
            panic!("id cannot be change");
        }
        self.id = Some(id);
    }

    pub fn name(&self) -> &TableName {
        &self.name
    }

    pub fn columns(&self) -> &Vec<ColumnId> {
        &self.columns
    }

    // テーブル名の変更
    pub fn change_name(&mut self, new_name: TableName) {
        self.name = new_name;
    }

    // カラムの移動(指定したカラムを指定したカラムの前に移動)
    pub fn move_columns_in_front_of(
        &mut self,
        target: &ColumnId,
        destination: &ColumnId,
    ) -> anyhow::Result<(), TableEntityError> {
        let target_found: Option<(usize, &ColumnId)> = self
            .columns
            .iter()
            .enumerate()
            .find(|&(_, column_id)| column_id == target);
        let target_index = match target_found {
            Some((index, _)) => index,
            None => {
                return Err(TableEntityError::ColumnNotFound(target.clone()));
            }
        };
        let destination_found: Option<(usize, &ColumnId)> = self
            .columns
            .iter()
            .enumerate()
            .find(|&(_, column_id)| column_id == destination);
        match destination_found {
            Some((destination_index, _)) => {
                if destination_index < target_index {
                    let target = self.columns.remove(target_index);
                    self.columns.insert(destination_index, target);
                } else if target_index < destination_index {
                    self.columns.insert(destination_index, target.clone());
                    self.columns.remove(target_index);
                }
                Ok(())
            }
            None => {
                return Err(TableEntityError::ColumnNotFound(destination.clone()));
            }
        }
    }

    // カラムの移動(指定したカラムを指定したカラムの後ろに移動)
    pub fn move_columns_behind(
        &mut self,
        target: &ColumnId,
        destination: &ColumnId,
    ) -> anyhow::Result<(), TableEntityError> {
        let target_found: Option<(usize, &ColumnId)> = self
            .columns
            .iter()
            .enumerate()
            .find(|&(_, column_id)| column_id == target);
        let target_index = match target_found {
            Some((index, _)) => index,
            None => {
                return Err(TableEntityError::ColumnNotFound(target.clone()));
            }
        };
        let destination_found: Option<(usize, &ColumnId)> = self
            .columns
            .iter()
            .enumerate()
            .find(|&(_, column_id)| column_id == destination);
        match destination_found {
            Some((destination_index, _)) => {
                if destination_index < target_index {
                    let target = self.columns.remove(target_index);
                    self.columns.insert(destination_index + 1, target);
                } else if target_index < destination_index {
                    self.columns.insert(destination_index + 1, target.clone());
                    self.columns.remove(target_index);
                }
                Ok(())
            }
            None => {
                return Err(TableEntityError::ColumnNotFound(destination.clone()));
            }
        }
    }

    // カラムの挿入（指定したカラムの前に挿入）
    pub fn insert_column_in_front_of(
        &mut self,
        destination: &ColumnId,
        new_column: ColumnId,
    ) -> anyhow::Result<(), TableEntityError> {
        let target_found: Option<(usize, &ColumnId)> = self
            .columns
            .iter()
            .enumerate()
            .find(|&(_, column_id)| column_id == destination);
        match target_found {
            Some((index, _)) => Ok(self.columns.insert(index, new_column)),
            None => Err(TableEntityError::ColumnNotFound(destination.clone())),
        }
    }

    // カラムの挿入（指定したカラムの後ろに挿入）
    pub fn insert_column_behind(
        &mut self,
        destination: &ColumnId,
        new_column: ColumnId,
    ) -> anyhow::Result<(), TableEntityError> {
        let target_found: Option<(usize, &ColumnId)> = self
            .columns
            .iter()
            .enumerate()
            .find(|&(_, column_id)| column_id == destination);
        match target_found {
            Some((index, _)) => Ok(self.columns.insert(index + 1, new_column)),
            None => Err(TableEntityError::ColumnNotFound(destination.clone())),
        }
    }

    // カラムの削除
    pub fn remove_column(&mut self, column_id: &ColumnId) {
        self.columns.retain(|id| id != column_id);
    }
}

impl Entity for Table {
    type Identity = TableId;

    fn identity(&self) -> &Self::Identity {
        self.id.as_ref().expect("id must be set before comparison")
    }
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        Entity::eq(self, other)
    }
}

#[derive(Debug, Error)]
pub enum TableEntityError {
    #[error("column list is empty")]
    EmptyColumnList,
    #[error("column not found, column_id: {0}")]
    ColumnNotFound(ColumnId),
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{
            column::column_id::ColumnId,
            table::{table_id::TableId, table_name::TableName},
        },
        shared::value_object::ValueObject,
    };

    use super::{Table, TableEntityError};

    #[test]
    fn test_move_columns() -> anyhow::Result<()> {
        let table_id = TableId::new("table".to_string())?;
        let table_name = TableName::new("table".to_string())?;
        let column1 = ColumnId::new("column1".to_string())?;
        let column2 = ColumnId::new("column2".to_string())?;
        let column3 = ColumnId::new("column3".to_string())?;
        let column4 = ColumnId::new("column4".to_string())?;
        let column5 = ColumnId::new("column5".to_string())?;

        let mut table = Table::reconstruct(
            table_id,
            table_name,
            vec![
                column1.clone(),
                column2.clone(),
                column3.clone(),
                column4.clone(),
                column5.clone(),
            ],
        )?;

        // 1 を 3 に移動
        table.move_columns_in_front_of(&column1, &column3)?;
        assert_eq!(
            table.columns(),
            &vec![
                column2.clone(),
                column1.clone(),
                column3.clone(),
                column4.clone(),
                column5.clone()
            ]
        );

        // 1 を 5 に移動
        table.move_columns_in_front_of(&column1, &column5)?;
        assert_eq!(
            table.columns(),
            &vec![
                column2.clone(),
                column3.clone(),
                column4.clone(),
                column1.clone(),
                column5.clone()
            ]
        );

        // 1 を 2 に移動
        table.move_columns_in_front_of(&column1, &column2)?;
        assert_eq!(
            table.columns(),
            &vec![
                column1.clone(),
                column2.clone(),
                column3.clone(),
                column4.clone(),
                column5.clone()
            ]
        );

        // 1 を 1 に移動
        table.move_columns_in_front_of(&column1, &column1)?;
        assert_eq!(
            table.columns(),
            &vec![
                column1.clone(),
                column2.clone(),
                column3.clone(),
                column4.clone(),
                column5.clone()
            ]
        );

        Ok(())
    }

    #[test]
    fn test_move_columns_target_not_found() -> anyhow::Result<()> {
        let table_id = TableId::new("table".to_string())?;
        let table_name = TableName::new("table".to_string())?;
        let column1 = ColumnId::new("column1".to_string())?;
        let column2 = ColumnId::new("column2".to_string())?;
        let column3 = ColumnId::new("column3".to_string())?;
        let column4 = ColumnId::new("column4".to_string())?;
        let column5 = ColumnId::new("column5".to_string())?;

        let mut table = Table::reconstruct(
            table_id,
            table_name,
            vec![
                column1.clone(),
                column2.clone(),
                column3.clone(),
                column4.clone(),
                column5.clone(),
            ],
        )?;

        // 6 を 3 に移動
        let result =
            table.move_columns_in_front_of(&ColumnId::new("column6".to_string())?, &column3);
        if let Err(TableEntityError::ColumnNotFound(column_id)) = result {
            assert_eq!(column_id.value(), "column6");
        } else {
            panic!("unexpected result");
        }
        Ok(())
    }

    #[test]
    fn test_move_columns_destination_not_found() -> anyhow::Result<()> {
        let table_id = TableId::new("table".to_string())?;
        let table_name = TableName::new("table".to_string())?;
        let column1 = ColumnId::new("column1".to_string())?;
        let column2 = ColumnId::new("column2".to_string())?;
        let column3 = ColumnId::new("column3".to_string())?;
        let column4 = ColumnId::new("column4".to_string())?;
        let column5 = ColumnId::new("column5".to_string())?;

        let mut table = Table::reconstruct(
            table_id,
            table_name,
            vec![
                column1.clone(),
                column2.clone(),
                column3.clone(),
                column4.clone(),
                column5.clone(),
            ],
        )?;

        // 1 を 6 に移動
        let result =
            table.move_columns_in_front_of(&column1, &ColumnId::new("column6".to_string())?);
        if let Err(TableEntityError::ColumnNotFound(column_id)) = result {
            assert_eq!(column_id.value(), "column6");
        } else {
            panic!("unexpected result");
        }
        Ok(())
    }

    #[test]
    fn test_move_columns_behind() -> anyhow::Result<()> {
        let table_id = TableId::new("table_id".to_string())?;
        let table_name = TableName::new("table_name".to_string())?;
        let column1 = ColumnId::new("column_id1".to_string())?;
        let column2 = ColumnId::new("column_id2".to_string())?;
        let column3 = ColumnId::new("column_id3".to_string())?;
        let column4 = ColumnId::new("column_id4".to_string())?;
        let column5 = ColumnId::new("column_id5".to_string())?;

        // 1, 2, 3, 4, 5
        let mut table = Table::reconstruct(
            table_id,
            table_name,
            vec![
                column1.clone(),
                column2.clone(),
                column3.clone(),
                column4.clone(),
                column5.clone(),
            ],
        )?;

        // 1 を 3 の後ろに移動
        table.move_columns_behind(&column1, &column3)?;

        // 2, 3, 1, 4, 5
        assert_eq!(
            table.columns(),
            &vec![
                column2.clone(),
                column3.clone(),
                column1.clone(),
                column4.clone(),
                column5.clone()
            ]
        );

        // 1 を 5 の後ろに移動
        table.move_columns_behind(&column1, &column5)?;

        // 2, 3, 4, 5, 1
        assert_eq!(
            table.columns(),
            &vec![
                column2.clone(),
                column3.clone(),
                column4.clone(),
                column5.clone(),
                column1.clone()
            ]
        );

        // 1 を 2 の後ろに移動
        table.move_columns_behind(&column1, &column2)?;

        // 2, 1, 3, 4, 5
        assert_eq!(
            table.columns(),
            &vec![
                column2.clone(),
                column1.clone(),
                column3.clone(),
                column4.clone(),
                column5.clone()
            ]
        );

        // 1 を 1 の後ろに移動
        table.move_columns_behind(&column1, &column1)?;

        // 2, 1, 3, 4, 5
        assert_eq!(
            table.columns(),
            &vec![
                column2.clone(),
                column1.clone(),
                column3.clone(),
                column4.clone(),
                column5.clone()
            ]
        );

        Ok(())
    }

    #[test]
    fn test_move_columns_behind_target_not_found() -> anyhow::Result<()> {
        let table_id = TableId::new("table_id".to_string())?;
        let table_name = TableName::new("table_name".to_string())?;
        let column1 = ColumnId::new("column_id1".to_string())?;
        let column2 = ColumnId::new("column_id2".to_string())?;
        let column3 = ColumnId::new("column_id3".to_string())?;
        let column4 = ColumnId::new("column_id4".to_string())?;
        let column5 = ColumnId::new("column_id5".to_string())?;

        // 1, 2, 3, 4, 5
        let mut table = Table::reconstruct(
            table_id,
            table_name,
            vec![
                column1.clone(),
                column2.clone(),
                column3.clone(),
                column4.clone(),
                column5.clone(),
            ],
        )?;

        // 6 を 3 の後ろに移動
        let result = table.move_columns_behind(&ColumnId::new("column_id6".to_string())?, &column3);
        if let Err(TableEntityError::ColumnNotFound(column_id)) = result {
            assert_eq!(column_id.value(), "column_id6");
        } else {
            panic!("unexpected result");
        }
        Ok(())
    }

    #[test]
    fn test_move_columns_behind_destination_not_found() {
        let table_id = TableId::new("table_id".to_string()).unwrap();
        let table_name = TableName::new("table_name".to_string()).unwrap();
        let column1 = ColumnId::new("column_id1".to_string()).unwrap();
        let column2 = ColumnId::new("column_id2".to_string()).unwrap();
        let column3 = ColumnId::new("column_id3".to_string()).unwrap();
        let column4 = ColumnId::new("column_id4".to_string()).unwrap();
        let column5 = ColumnId::new("column_id5".to_string()).unwrap();

        // 1, 2, 3, 4, 5
        let mut table = Table::reconstruct(
            table_id,
            table_name,
            vec![
                column1.clone(),
                column2.clone(),
                column3.clone(),
                column4.clone(),
                column5.clone(),
            ],
        )
        .unwrap();

        // 1 を 6 の後ろに移動
        let result =
            table.move_columns_behind(&column1, &ColumnId::new("column_id6".to_string()).unwrap());
        if let Err(TableEntityError::ColumnNotFound(column_id)) = result {
            assert_eq!(column_id.value(), "column_id6");
        } else {
            panic!("unexpected result");
        }
    }
}
