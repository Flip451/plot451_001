use crate::models::column::column::Column;

use super::{column_directory::ColumnDirectory, column_directory_id::ColumnDirectoryId};

pub struct DirectoryContents {
    directory_id: ColumnDirectoryId,
    columns: Vec<Column>,
    directories: Vec<ColumnDirectory>,
}

impl DirectoryContents {
    pub fn new(
        parent_directory: &ColumnDirectory,
        columns: Vec<Column>,
        child_directories: Vec<ColumnDirectory>,
    ) -> Self {
        columns.iter().for_each(|column| {
            assert_eq!(column.directory_id(), parent_directory.id());
        });
        child_directories.iter().for_each(|directory| {
            assert_eq!(directory.parent().as_ref().unwrap(), parent_directory.id());
        });

        Self {
            directory_id: parent_directory.id().clone(),
            columns,
            directories: child_directories,
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
