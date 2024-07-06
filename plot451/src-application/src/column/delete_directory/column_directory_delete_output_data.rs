use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ColumnDirectoryDeleteOutputData {}

impl ColumnDirectoryDeleteOutputData {
    pub fn new() -> Self {
        Self {}
    }
}
