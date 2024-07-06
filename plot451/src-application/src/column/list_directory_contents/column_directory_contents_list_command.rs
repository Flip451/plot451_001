use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ColumnDirectoryContentsListCommand {
    pub(super) directory_id: String,
}