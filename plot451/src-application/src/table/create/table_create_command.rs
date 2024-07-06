use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TableCreateCommand {
    pub(super) table_name: String,
    pub(super) column_ids: Vec<String>,
}
