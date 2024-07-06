use crate::models::column::column::Column;

use super::{column_directory::ColumnDirectory, column_directory_id::ColumnDirectoryId};

pub struct DirectoryContents {
    directory_id: ColumnDirectoryId,
    columns: Vec<Column>,
    directories: Vec<ColumnDirectory>,
}

impl DirectoryContents {
    pub fn new(
        directory: &ColumnDirectory,
        columns: Vec<Column>,
        directories: Vec<ColumnDirectory>,
    ) -> Self {
        columns.iter().for_each(|column| {
            assert_eq!(column.directory_id(), directory.id());
        });
        directories.iter().for_each(|directory| {
            assert_eq!(directory.parent().as_ref().unwrap(), directory.id());
        });

        Self {
            directory_id: directory.id().clone(),
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
