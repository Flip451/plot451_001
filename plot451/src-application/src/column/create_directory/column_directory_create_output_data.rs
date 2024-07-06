use serde::{Deserialize, Serialize};

use src_domain::{
    models::column::column_directory::column_directory::ColumnDirectory,
    shared::value_object::ValueObject,
};

#[derive(Serialize, Deserialize)]
pub struct ColumnDirectoryCreateOutputData {
    pub(super) directory_id: String,
    pub(super) directory_name: String,
    pub(super) parent_id: Option<String>,
}

impl ColumnDirectoryCreateOutputData {
    pub(super) fn new(source: ColumnDirectory) -> Self {
        Self {
            directory_id: source.id().clone_value(),
            directory_name: source.name().clone_value(),
            parent_id: source.parent().as_ref().map(|id| id.clone_value()),
        }
    }
}
