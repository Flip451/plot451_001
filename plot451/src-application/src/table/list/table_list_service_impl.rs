use src_domain::models::{
    column::{column_repository::IColumnRepository, column_with_cells::ColumnWithCells},
    table::{
        table_repository::ITableRepository, table_with_columns_and_cells::TableWithColumnsAndCells,
    },
};

use super::{
    table_list_command::TableListCommand,
    table_list_output_data::TableListOutputData,
    table_list_service::{ITableListService, TableListServiceError, TableListServiceResult},
};

pub struct TableListService<'a, 'b, CR, TR>
where
    CR: IColumnRepository,
    TR: ITableRepository,
{
    column_repository: &'a CR,
    table_repository: &'b TR,
}

impl<'a, 'b, CR, TR> TableListService<'a, 'b, CR, TR>
where
    CR: IColumnRepository,
    TR: ITableRepository,
{
    pub fn new(column_repository: &'a CR, table_repository: &'b TR) -> Self {
        TableListService {
            column_repository,
            table_repository,
        }
    }
}

impl<'a, 'b, CR, TR> ITableListService for TableListService<'a, 'b, CR, TR>
where
    CR: IColumnRepository,
    TR: ITableRepository,
{
    async fn handle(
        &self,
        _command: TableListCommand,
    ) -> TableListServiceResult<TableListOutputData> {
        let tables = self
            .table_repository
            .find_all()
            .await
            .map_err(|e| TableListServiceError::TableRepositoryError(e))?;

        let mut list_of_table_with_columns_and_cell: Vec<TableWithColumnsAndCells> = vec![];
        for table in tables.iter() {
            let table_id = table.id().as_ref().unwrap().clone();
            let table_name = table.name().clone();
            let column_ids = table.columns();
            let columns = self
                .column_repository
                .find_by_ids(&column_ids)
                .await
                .map_err(|e| TableListServiceError::ColumnRepositoryError(e))?;

            let mut list_of_column_with_cells: Vec<ColumnWithCells> = vec![];
            for column in columns.iter() {
                let column_id = column.id().as_ref().unwrap().clone();
                let column_name = column.name().clone();
                let cell_ids = column.cells();
                let cells = self
                    .column_repository
                    .find_cells_by_ids(&cell_ids)
                    .await
                    .map_err(|e| TableListServiceError::ColumnRepositoryError(e))?;
                let column_with_cells = ColumnWithCells::new(column_id, column_name, cells);
                list_of_column_with_cells.push(column_with_cells);
            }
            let table_with_columns_and_cells =
                TableWithColumnsAndCells::new(table_id, table_name, list_of_column_with_cells);
            list_of_table_with_columns_and_cell.push(table_with_columns_and_cells)
        }

        let output_data = TableListOutputData::new(list_of_table_with_columns_and_cell);
        Ok(output_data)
    }
}

#[cfg(test)]
mod tests {
    use src_in_memory_infrastructure::{
        column::in_memory_column_repository::InMemoryColumnRepository,
        table::in_memory_table_repository::InMemoryTableRepository,
    };

    use crate::table::list::{
        table_list_command::TableListCommand,
        table_list_output_data::{ColumnCellInOutputData, ColumnInOutputData, TableInOutputData},
    };
    use src_domain::{
        models::{
            column::{
                column::Column,
                column_cell::{
                    column_cell::ColumnCell, column_cell_id::ColumnCellId,
                    column_cell_value::ColumnCellValue,
                },
                column_directory::column_directory_id::ColumnDirectoryId,
                column_id::ColumnId,
                column_name::ColumnName,
            },
            table::{table::Table, table_id::TableId, table_name::TableName},
        },
        shared::value_object::ValueObject,
    };

    use super::*;

    #[tokio::test]
    async fn test_handle() -> anyhow::Result<()> {
        let column_repository = InMemoryColumnRepository::new();
        let table_repository = InMemoryTableRepository::new();

        // 事前にセルを作成しておく
        let cell_id1 = ColumnCellId::new("cell_id_1".to_string())?;
        let cell1 = ColumnCell::new(Some(cell_id1.clone()), ColumnCellValue::new(Some(1.0))?);

        let cell_id2 = ColumnCellId::new("cell_id_2".to_string())?;
        let cell2 = ColumnCell::new(Some(cell_id2.clone()), ColumnCellValue::new(Some(2.0))?);

        let cell_id3 = ColumnCellId::new("cell_id_3".to_string())?;
        let cell3 = ColumnCell::new(Some(cell_id3.clone()), ColumnCellValue::new(Some(3.0))?);

        let cell_id4 = ColumnCellId::new("cell_id_4".to_string())?;
        let cell4 = ColumnCell::new(Some(cell_id4.clone()), ColumnCellValue::new(None)?);

        let cell_id5 = ColumnCellId::new("cell_id_5".to_string())?;
        let cell5 = ColumnCell::new(Some(cell_id5.clone()), ColumnCellValue::new(Some(5.0))?);

        let cell_id6 = ColumnCellId::new("cell_id_6".to_string())?;
        let cell6 = ColumnCell::new(Some(cell_id6.clone()), ColumnCellValue::new(Some(6.0))?);

        let cell_id7 = ColumnCellId::new("cell_id_7".to_string())?;
        let cell7 = ColumnCell::new(Some(cell_id7.clone()), ColumnCellValue::new(Some(7.0))?);

        // 事前にカラムを作成しておく
        let column_id1 = ColumnId::new("column_id_1".to_string())?;
        let column1 = Column::new(
            Some(column_id1.clone()),
            ColumnName::new("column_name_1".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![
                cell1.id().as_ref().unwrap().clone(),
                cell2.id().as_ref().unwrap().clone(),
            ],
        );

        let column_id2 = ColumnId::new("column_id_2".to_string())?;
        let column2 = Column::new(
            Some(column_id2.clone()),
            ColumnName::new("column_name_2".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![
                cell3.id().as_ref().unwrap().clone(),
                cell4.id().as_ref().unwrap().clone(),
            ],
        );

        let column_id3 = ColumnId::new("column_id_3".to_string())?;
        let column3 = Column::new(
            Some(column_id3.clone()),
            ColumnName::new("column_name_3".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![
                cell5.id().as_ref().unwrap().clone(),
                cell6.id().as_ref().unwrap().clone(),
                cell7.id().as_ref().unwrap().clone(),
            ],
        );

        // 事前にテーブルを作成しておく
        let table_id1 = TableId::new("table_id_1".to_string())?;
        let table1 = Table::new(
            Some(table_id1.clone()),
            TableName::new("table_name_1".to_string())?,
            vec![
                column1.id().as_ref().unwrap().clone(),
                column2.id().as_ref().unwrap().clone(),
            ],
        )?;

        let table_id2 = TableId::new("table_id_2".to_string())?;
        let table2 = Table::new(
            Some(table_id2.clone()),
            TableName::new("table_name_2".to_string())?,
            vec![
                column2.id().as_ref().unwrap().clone(),
                column3.id().as_ref().unwrap().clone(),
            ],
        )?;

        // テーブル・カラム・セルをリポジトリに保存
        column_repository.save_cell(&cell1).await?;
        column_repository.save_cell(&cell2).await?;
        column_repository.save_cell(&cell3).await?;
        column_repository.save_cell(&cell4).await?;
        column_repository.save_cell(&cell5).await?;
        column_repository.save_cell(&cell6).await?;
        column_repository.save_cell(&cell7).await?;
        column_repository.save(&column1).await?;
        column_repository.save(&column2).await?;
        column_repository.save(&column3).await?;
        table_repository.save(&table1).await?;
        table_repository.save(&table2).await?;

        // サービスのインスタンス化
        let service = TableListService::new(&column_repository, &table_repository);

        // コマンドの作成
        let command = TableListCommand {};

        // サービスの実行
        let TableListOutputData { tables } = service.handle(command).await?;

        let expected = vec![
            TableInOutputData {
                table_id: table1.id().as_ref().unwrap().clone_value(),
                table_name: table1.name().clone_value(),
                columns: vec![
                    ColumnInOutputData {
                        column_id: column1.id().as_ref().unwrap().clone_value(),
                        column_name: column1.name().clone_value(),
                        cells: vec![
                            ColumnCellInOutputData {
                                cell_id: cell1.id().as_ref().unwrap().clone_value(),
                                cell_value: cell1.cell_value().clone_value(),
                            },
                            ColumnCellInOutputData {
                                cell_id: cell2.id().as_ref().unwrap().clone_value(),
                                cell_value: cell2.cell_value().clone_value(),
                            },
                        ],
                    },
                    ColumnInOutputData {
                        column_id: column2.id().as_ref().unwrap().clone_value(),
                        column_name: column2.name().clone_value(),
                        cells: vec![
                            ColumnCellInOutputData {
                                cell_id: cell3.id().as_ref().unwrap().clone_value(),
                                cell_value: cell3.cell_value().clone_value(),
                            },
                            ColumnCellInOutputData {
                                cell_id: cell4.id().as_ref().unwrap().clone_value(),
                                cell_value: cell4.cell_value().clone_value(),
                            },
                        ],
                    },
                ],
            },
            TableInOutputData {
                table_id: table2.id().as_ref().unwrap().clone_value(),
                table_name: table2.name().clone_value(),
                columns: vec![
                    ColumnInOutputData {
                        column_id: column2.id().as_ref().unwrap().clone_value(),
                        column_name: column2.name().clone_value(),
                        cells: vec![
                            ColumnCellInOutputData {
                                cell_id: cell3.id().as_ref().unwrap().clone_value(),
                                cell_value: cell3.cell_value().clone_value(),
                            },
                            ColumnCellInOutputData {
                                cell_id: cell4.id().as_ref().unwrap().clone_value(),
                                cell_value: cell4.cell_value().clone_value(),
                            },
                        ],
                    },
                    ColumnInOutputData {
                        column_id: column3.id().as_ref().unwrap().clone_value(),
                        column_name: column3.name().clone_value(),
                        cells: vec![
                            ColumnCellInOutputData {
                                cell_id: cell5.id().as_ref().unwrap().clone_value(),
                                cell_value: cell5.cell_value().clone_value(),
                            },
                            ColumnCellInOutputData {
                                cell_id: cell6.id().as_ref().unwrap().clone_value(),
                                cell_value: cell6.cell_value().clone_value(),
                            },
                            ColumnCellInOutputData {
                                cell_id: cell7.id().as_ref().unwrap().clone_value(),
                                cell_value: cell7.cell_value().clone_value(),
                            },
                        ],
                    },
                ],
            },
        ];

        for expected_table in expected.iter() {
            assert!(tables
                .iter()
                .find(|table| table == &expected_table)
                .is_some());
        }
        // 実行結果の評価
        Ok(())
    }
}
