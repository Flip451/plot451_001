use std::collections::HashSet;

use super::column_cell::column_cell_id::ColumnCellId;
use super::column_directory::column_directory_id::ColumnDirectoryId;
use super::column_id::ColumnId;
use super::column_name::ColumnName;
use crate::shared::entity::Entity;

// entity
#[derive(Debug, Clone, Eq, Hash)]
pub struct Column {
    id: Option<ColumnId>,
    name: ColumnName,
    directory: ColumnDirectoryId,
    cells: Vec<ColumnCellId>,
}

impl Column {
    // Column の再構築
    pub fn new(
        id: Option<ColumnId>,
        name: ColumnName,
        directory: ColumnDirectoryId,
        cells: Vec<ColumnCellId>,
    ) -> Self {
        Self {
            id,
            name,
            directory,
            cells,
        }
    }

    // getter & setter
    pub fn id(&self) -> &Option<ColumnId> {
        &self.id
    }

    pub fn set_id(&mut self, id: ColumnId) {
        if let Some(_) = self.id {
            panic!("id cannot be change");
        }
        self.id = Some(id);
    }

    pub fn name(&self) -> &ColumnName {
        &self.name
    }

    pub fn cells(&self) -> &Vec<ColumnCellId> {
        &self.cells
    }

    // カラム名の変更
    pub fn change_name(&mut self, new_name: ColumnName) {
        self.name = new_name;
    }

    // セルの挿入
    pub fn insert_cells(&mut self, cell_id: ColumnCellId) {
        self.cells.push(cell_id);
    }

    // セルの削除
    pub fn remove_cells(&mut self, cell_id: &ColumnCellId) {
        self.cells.retain(|c| c != cell_id);
    }

    // セルの順序変更
    pub fn change_order(
        &mut self,
        new_order: Vec<ColumnCellId>,
    ) -> anyhow::Result<(), ColumnEntityError> {
        let old_order_set: HashSet<&ColumnCellId> = self.cells.iter().collect();
        let new_order_set: HashSet<&ColumnCellId> = new_order.iter().collect();
        if old_order_set.len() != new_order_set.len()
            || old_order_set.difference(&new_order_set).count() != 0
            || new_order_set.difference(&old_order_set).count() != 0
        {
            return Err(ColumnEntityError::InvalidOrder);
        }
        self.cells = new_order;
        Ok(())
    }

    // ディレクトリの移動
    pub fn move_to(&mut self, new_directory: ColumnDirectoryId) {
        self.directory = new_directory;
    }
}

impl Entity for Column {
    type Identity = ColumnId;

    fn identity(&self) -> &Self::Identity {
        self.id.as_ref().unwrap()
    }
}

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        Entity::eq(self, other)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ColumnEntityError {
    #[error("invalid order")]
    InvalidOrder,
}

#[cfg(test)]
mod tests {
    use crate::models::column::column_directory::column_directory_id::ColumnDirectoryId;
    use crate::models::column::column_id::ColumnId;
    use crate::models::column::column_name::ColumnName;
    use crate::shared::value_object::ValueObject;

    use super::super::column_cell::column_cell_id::ColumnCellId;

    use super::Column;

    impl Column {
        pub fn get_directory_id(&self) -> &ColumnDirectoryId {
            &self.directory
        }
    }

    #[test]
    fn test_change_order() {
        let column_id = ColumnId::new("column_id".to_string()).unwrap();
        let column_name = ColumnName::new("column_name".to_string()).unwrap();
        let directory_id = ColumnDirectoryId::new("0".to_string()).unwrap();
        let cell_id1 = ColumnCellId::new("cell_id1".to_string()).unwrap();
        let cell_id2 = ColumnCellId::new("cell_id2".to_string()).unwrap();
        let cell_id3 = ColumnCellId::new("cell_id3".to_string()).unwrap();
        let mut column = Column::new(
            Some(column_id),
            column_name,
            directory_id,
            vec![cell_id1.clone(), cell_id2.clone(), cell_id3.clone()],
        );

        let new_order = vec![cell_id3.clone(), cell_id1.clone(), cell_id2.clone()];
        assert!(column.change_order(new_order.clone()).is_ok());
        assert_eq!(column.cells(), &new_order);

        let new_order = vec![cell_id3.clone(), cell_id1.clone()];
        assert!(column.change_order(new_order).is_err());

        let new_order = vec![cell_id3.clone(), cell_id1.clone(), cell_id3.clone()];
        assert!(column.change_order(new_order).is_err());

        let cell_id4 = ColumnCellId::new("cell_id4".to_string()).unwrap();
        let new_order = vec![cell_id3.clone(), cell_id1.clone(), cell_id4.clone()];
        assert!(column.change_order(new_order).is_err());
    }
}
