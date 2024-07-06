use crate::models::column::column_with_cells::ColumnWithCells;

use super::{table_id::TableId, table_name::TableName};

pub struct TableWithColumnsAndCells {
    id: TableId,
    name: TableName,
    columns: Vec<ColumnWithCells>,
}

impl TableWithColumnsAndCells {
    pub fn new(id: TableId, name: TableName, columns: Vec<ColumnWithCells>) -> Self {
        // TODO: assert 文を追加
        Self { id, name, columns }
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