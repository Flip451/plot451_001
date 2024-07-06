use src_domain::models::column::{
    column::Column,
    column_cell::{
        column_cell::ColumnCell, column_cell_id::ColumnCellId, column_cell_value::ColumnCellValue,
    },
    column_directory::{
        column_directory::ColumnDirectory, column_directory_id::ColumnDirectoryId,
        column_directory_name::ColumnDirectoryName,
    },
    column_factory::{ColumnFactoryResult, IColumnFactory},
    column_name::ColumnName,
};

pub struct InMemoryColumnFactory {}

impl InMemoryColumnFactory {
    pub fn new() -> Self {
        InMemoryColumnFactory {}
    }
}

impl IColumnFactory for InMemoryColumnFactory {
    async fn create_column(
        &self,
        name: ColumnName,
        directory: ColumnDirectoryId,
        cells: Vec<ColumnCellId>,
    ) -> ColumnFactoryResult<Column> {
        let column = Column::new(None, name, directory, cells);
        Ok(column)
    }

    async fn create_cell(&self, value: ColumnCellValue) -> ColumnFactoryResult<ColumnCell> {
        let cell = ColumnCell::new(None, value);
        Ok(cell)
    }

    async fn create_directory(
        &self,
        name: ColumnDirectoryName,
        parent_id: Option<ColumnDirectoryId>,
    ) -> ColumnFactoryResult<ColumnDirectory> {
        let directory = ColumnDirectory::new(None, name, parent_id);
        Ok(directory)
    }
}
