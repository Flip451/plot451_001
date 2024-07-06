use super::{column_cell::column_cell::ColumnCell, column_id::ColumnId, column_name::ColumnName};

pub struct ColumnWithCells {
    column_id: ColumnId,
    column_name: ColumnName,
    cells: Vec<ColumnCell>
}

impl ColumnWithCells {
    pub fn new(column_id: ColumnId, column_name: ColumnName, cells: Vec<ColumnCell>) -> Self {
        // TODO: assert 文を追加
        Self {
            column_id,
            column_name,
            cells
        }
    }

    pub fn id(&self) -> &ColumnId {
        &self.column_id
    }

    pub fn name(&self) -> &ColumnName {
        &self.column_name
    }

    pub fn cells(&self) -> &Vec<ColumnCell> {
        &self.cells
    }
}