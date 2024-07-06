use serde::{Deserialize, Serialize};

use src_domain::{
    models::column::column_directory::directory_contents::DirectoryContents,
    shared::value_object::ValueObject,
};

#[derive(Deserialize, Serialize)]
pub struct ColumnDirectoryContentsListOutputData {
    pub(super) columns: Vec<ColumnInOutputData>,
    pub(super) directories: Vec<DirectoryInOutputData>,
}

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, Debug, Clone)]
pub(super) struct ColumnInOutputData {
    pub(super) column_id: String,
    pub(super) column_name: String,
}

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, Debug, Clone)]
pub(super) struct DirectoryInOutputData {
    pub(super) directory_id: String,
    pub(super) directory_name: String,
}

impl ColumnDirectoryContentsListOutputData {
    pub fn new(source: DirectoryContents) -> Self {
        let columns = source
            .columns()
            .iter()
            .map(|column| ColumnInOutputData {
                column_id: column.id().clone_value(),
                column_name: column.name().clone_value(),
            })
            .collect();
        let directories = source
            .directories()
            .iter()
            .map(|directory| DirectoryInOutputData {
                directory_id: directory.id().clone_value(),
                directory_name: directory.name().clone_value(),
            })
            .collect();
        ColumnDirectoryContentsListOutputData {
            columns,
            directories,
        }
    }
}
