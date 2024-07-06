use crate::models::column::column::Column;

use super::{column_directory::ColumnDirectory, column_directory_id::ColumnDirectoryId};

pub struct DirectoryContents {
    directory_id: ColumnDirectoryId,
    columns: Vec<Column>,
    directories: Vec<ColumnDirectory>,
}

impl DirectoryContents {
    pub fn new(
        directory_id: ColumnDirectoryId,
        columns: Vec<Column>,
        directories: Vec<ColumnDirectory>,
    ) -> Self {
        // TODO: assert 文を追加
        Self {
            directory_id,
            columns,
            directories,
        }
    }

    pub fn directory_id(&self) -> &ColumnDirectoryId {
        &self.directory_id
    }

    pub fn columns(&self) -> &Vec<Column> {
        &self.columns
    }

    pub fn directories(&self) -> &Vec<ColumnDirectory> {
        &self.directories
    }
}
