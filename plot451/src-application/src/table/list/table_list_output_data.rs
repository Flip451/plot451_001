use serde::{Deserialize, Serialize};

use src_domain::{
    models::table::table_with_columns_and_cells::TableWithColumnsAndCells,
    shared::value_object::ValueObject,
};

#[derive(Deserialize, Serialize)]
pub struct TableListOutputData {
    pub(super) tables: Vec<TableInOutputData>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub(super) struct TableInOutputData {
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

impl TableListOutputData {
    pub(super) fn new(source: Vec<TableWithColumnsAndCells>) -> Self {
        TableListOutputData {
            tables: source
                .iter()
                .map(|table_with_columns_and_cells| TableInOutputData {
                    table_id: table_with_columns_and_cells.id().clone_value(),
                    table_name: table_with_columns_and_cells.name().clone_value(),
                    columns: table_with_columns_and_cells
                        .columns()
                        .iter()
                        .map(|column| ColumnInOutputData {
                            column_id: column.id().clone_value(),
                            column_name: column.name().clone_value(),
                            cells: column
                                .cells()
                                .iter()
                                .map(|cell| ColumnCellInOutputData {
                                    cell_id: cell.id().clone_value(),
                                    cell_value: cell.cell_value().clone_value(),
                                })
                                .collect(),
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}
