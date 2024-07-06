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
            table
                .columns()
                .iter()
                .map(|column_id| column_id)
                .collect::<Vec<_>>(),
            columns.iter().map(|column| column.id()).collect::<Vec<_>>()
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
