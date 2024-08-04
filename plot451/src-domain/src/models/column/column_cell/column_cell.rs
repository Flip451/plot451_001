use super::column_cell_id::ColumnCellId;
use super::column_cell_value::ColumnCellValue;
use crate::shared::entity::Entity;
use thiserror::Error;

// value object
#[derive(Debug, Clone)]
pub struct ColumnCell {
    id: Option<ColumnCellId>,
    cell_value: ColumnCellValue,
}

impl ColumnCell {
    pub fn new(cell_value: ColumnCellValue) -> Self {
        Self {
            id: None,
            cell_value,
        }
    }

    // ColumnCell の再構築
    pub fn reconstruct(id: ColumnCellId, cell_value: ColumnCellValue) -> Self {
        Self {
            id: Some(id),
            cell_value,
        }
    }

    // getter & setter
    pub fn id(&self) -> &ColumnCellId {
        &self.id.as_ref().expect("id is not set")
    }

    pub fn id_wrapped(&self) -> &Option<ColumnCellId> {
        &self.id
    }

    pub fn set_id(&mut self, id: ColumnCellId) {
        if let Some(_) = self.id {
            panic!("id cannot be change");
        }
        self.id = Some(id);
    }

    pub fn cell_value(&self) -> &ColumnCellValue {
        &self.cell_value
    }

    // セルの値の編集
    pub fn edit_cell_value(&mut self, cell_value: ColumnCellValue) {
        self.cell_value = cell_value;
    }
}

#[derive(Debug, Error)]
pub enum ColumnCellError {}

impl Entity for ColumnCell {
    type Identity = ColumnCellId;

    fn identity(&self) -> &Self::Identity {
        self.id.as_ref().unwrap()
    }
}

impl PartialEq for ColumnCell {
    fn eq(&self, other: &Self) -> bool {
        Entity::eq(self, other)
    }
}
