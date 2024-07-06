use std::{collections::HashSet, hash::RandomState};

use super::{
    column::Column, column_cell::column_cell::ColumnCell, column_id::ColumnId,
    column_name::ColumnName,
};

pub struct ColumnWithCells {
    column_id: ColumnId,
    column_name: ColumnName,
    cells: Vec<ColumnCell>,
}

impl ColumnWithCells {
    pub fn new(column: &Column, cells: Vec<ColumnCell>) -> Self {
        assert_eq!(
            HashSet::<_, RandomState>::from_iter(column.cells().iter()),
            HashSet::from_iter(cells.iter().map(|cell| cell.id()))
        );

        Self {
            column_id: column.id().clone(),
            column_name: column.name().clone(),
            cells,
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
