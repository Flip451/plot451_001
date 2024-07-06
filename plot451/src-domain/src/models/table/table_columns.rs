use crate::models::column::{column::Column, column_name::ColumnName};

use super::table::Table;

// ファーストクラスコレクション
pub struct TableColumns {
    columns: Vec<Column>,
}

impl TableColumns {
    pub fn new(table: &Table, columns: Vec<Column>) -> Self {
        assert_eq!(
            table
                .columns()
                .iter()
                .map(|column_id| column_id)
                .collect::<Vec<_>>(),
            columns.iter().map(|column| column.id()).collect::<Vec<_>>()
        );
        Self { columns }
    }

    pub(super) fn column_names(&self) -> Vec<&ColumnName> {
        self.columns.iter().map(|column| column.name()).collect()
    }
}
