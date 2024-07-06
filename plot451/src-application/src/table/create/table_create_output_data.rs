use serde::{Deserialize, Serialize};

use src_domain::{
    models::table::table_with_columns_and_cells::TableWithColumnsAndCells,
    shared::value_object::ValueObject,
};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct TableCreateOutputData {
    pub(super) table_id: String,
    pub(super) table_name: String,
    pub(super) columns: Vec<ColumnInOutputData>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub(super) struct ColumnInOutputData {
    pub(super) column_id: String,
    pub(super) column_name: String,
    pub(super) cells: Vec<ColumnCellInOutputData>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub(super) struct ColumnCellInOutputData {
    pub(super) cell_id: String,
    pub(super) cell_value: Option<f64>,
}
impl TableCreateOutputData {
    pub(super) fn new(source: TableWithColumnsAndCells) -> Self {
        Self {
            table_id: source.id().clone_value(),
            table_name: source.name().clone_value(),
            columns: source
                .columns()
                .iter()
                .map(|column| ColumnInOutputData {
                    column_id: column.id().clone_value(),
                    column_name: column.name().clone_value(),
                    cells: column
                        .cells()
                        .iter()
                        .map(|cell| ColumnCellInOutputData {
                            cell_id: cell.id().as_ref().unwrap().clone_value(),
                            cell_value: cell.cell_value().clone_value(),
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}
