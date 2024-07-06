use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ColumnCreateCommand {
    pub(super) name: String,
    pub(super) directory_id: String,
    pub(super) cells: Vec<Option<f64>>
}