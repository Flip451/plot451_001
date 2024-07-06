use serde::{Deserialize, Serialize};

use src_domain::{
    models::column::column_with_cells::ColumnWithCells, shared::value_object::ValueObject,
};

#[derive(Serialize, Deserialize)]
pub struct ColumnCreateOutputData {
    pub(super) column_id: String,
    pub(super) column_name: String,
    pub(super) cells: Vec<ColumnCellInOutputData>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct ColumnCellInOutputData {
    pub(super) cell_id: String,
    pub(super) cell_value: Option<f64>,
}

impl ColumnCreateOutputData {
    pub(super) fn new(source: ColumnWithCells) -> Self {
        Self {
            column_id: source.id().clone_value(),
            column_name: source.name().clone_value(),
            cells: source
                .cells()
                .into_iter()
                .map(|cell| ColumnCellInOutputData {
                    cell_id: cell.id().as_ref().unwrap().clone_value(),
                    cell_value: cell.cell_value().clone_value(),
                })
                .collect(),
        }
    }
}
