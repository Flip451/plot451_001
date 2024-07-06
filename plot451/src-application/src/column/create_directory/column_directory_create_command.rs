use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ColumnDirectoryCreateCommand {
    pub(super) name: String,
    pub(super) parent_id: Option<String>,
}
