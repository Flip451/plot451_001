use std::{collections::HashSet, hash::RandomState};

use crate::models::column::column_with_cells::ColumnWithCells;

use super::{table::Table, table_id::TableId, table_name::TableName};

pub struct TableWithColumnsAndCells {
    id: TableId,
    name: TableName,
    columns: Vec<ColumnWithCells>,
}

impl TableWithColumnsAndCells {
    pub fn new(table: &Table, columns: Vec<ColumnWithCells>) -> Self {
        assert_eq!(
            HashSet::<_, RandomState>::from_iter(table.columns().iter()),
            HashSet::from_iter(columns.iter().map(|column| column.id()))
        );

        Self {
            id: table.id().clone(),
            name: table.name().clone(),
            columns,
        }
    }

    pub fn id(&self) -> &TableId {
        &self.id
    }

    pub fn name(&self) -> &TableName {
        &self.name
    }

    pub fn columns(&self) -> &Vec<ColumnWithCells> {
        &self.columns
    }
}
