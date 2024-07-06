use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ColumnDirectoryDeleteCommand {
    pub(super) id: String,
}