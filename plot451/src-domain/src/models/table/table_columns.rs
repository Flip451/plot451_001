use crate::models::column::{column::Column, column_name::ColumnName};

// ファーストクラスコレクション
pub struct TableColumns {
    columns: Vec<Column>,
}

impl TableColumns {
    pub fn new(columns: Vec<Column>) -> Self {
        // TODO: assert 文を追加
        Self { columns }
    }

    pub(super) fn column_names(&self) -> Vec<&ColumnName> {
        self.columns.iter().map(|column| column.name()).collect()
    }
}
