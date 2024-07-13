use src_domain::models::{
    column::column_id::ColumnId,
    table::{
        table::Table,
        table_factory::{ITableFactory, TableFactoryError, TableFactoryResult},
        table_name::TableName,
    },
};

pub struct SqlxTableFactory {}

impl SqlxTableFactory {
    pub fn new() -> Self {
        SqlxTableFactory {}
    }
}

impl ITableFactory for SqlxTableFactory {
    async fn create_table(
        &self,
        name: TableName,
        columns: Vec<ColumnId>,
    ) -> TableFactoryResult<Table> {
        let table =
            Table::new(None, name, columns).map_err(|e| TableFactoryError::TableEntityError(e))?;
        Ok(table)
    }
}
