use src_domain::{
    models::{
        column::{
            column_id::ColumnId, column_repository::IColumnRepository,
            column_with_cells::ColumnWithCells,
        },
        table::{
            no_duplicated_column_names_specification::NoDuplicatedColumnNamesSpecification,
            table_columns::TableColumns, table_factory::ITableFactory, table_name::TableName,
            table_repository::ITableRepository,
            table_with_columns_and_cells::TableWithColumnsAndCells,
        },
    },
    shared::{specification::Specification, value_object::ValueObject},
};

use super::{
    table_create_command::TableCreateCommand,
    table_create_output_data::TableCreateOutputData,
    table_create_service::{
        ITableCreateService, TableCreateServiceError, TableCreateServiceResult,
    },
};

pub struct TableCreateService<'a, 'b, 'c, CR, TF, TR>
where
    CR: IColumnRepository,
    TF: ITableFactory,
    TR: ITableRepository,
{
    column_repository: &'a CR,
    table_factory: &'b TF,
    table_repository: &'c TR,
}

impl<'a, 'b, 'c, CR, TF, TR> TableCreateService<'a, 'b, 'c, CR, TF, TR>
where
    CR: IColumnRepository,
    TF: ITableFactory,
    TR: ITableRepository,
{
    pub fn new(column_repository: &'a CR, table_factory: &'b TF, table_repository: &'c TR) -> Self {
        TableCreateService {
            column_repository,
            table_factory,
            table_repository,
        }
    }
}

impl<'a, 'b, 'c, TF, TR, CR> ITableCreateService for TableCreateService<'a, 'b, 'c, CR, TF, TR>
where
    CR: IColumnRepository + Sync,
    TF: ITableFactory + Sync,
    TR: ITableRepository + Sync,
{
    // TODO: トランザクション処理を追加する
    async fn handle(
        &self,
        command: TableCreateCommand,
    ) -> TableCreateServiceResult<TableCreateOutputData> {
        // 値オブジェクトのインスタンス化
        let table_name = TableName::new(command.table_name)
            .map_err(|e| TableCreateServiceError::TableNameError(e))?;

        let mut column_ids = vec![];
        for column_id in command.column_ids {
            let column_id =
                ColumnId::new(column_id).map_err(|e| TableCreateServiceError::ColumnIdError(e))?;
            column_ids.push(column_id);
        }

        // ファーストクラスコレクション作成の準備
        // 指定したカラムIDのうちに、それに紐づくカラムが存在しないものがある場合にはエラーを返す
        let columns = self
            .column_repository
            .find_by_ids(&column_ids)
            .await
            .map_err(|e| TableCreateServiceError::ColumnRepositoryError(e))?;

        let mut columns_with_cells = vec![];
        for column in columns.iter() {
            let cells = self
                .column_repository
                .find_cells_by_column_id(column.id())
                .await
                .map_err(|e| TableCreateServiceError::ColumnRepositoryError(e))?;
            let column_with_cells = ColumnWithCells::new(column, cells);
            columns_with_cells.push(column_with_cells);
        }

        // エンティティのインスタンス化
        let mut table = self
            .table_factory
            .create_table(table_name, column_ids)
            .await
            .map_err(|e| TableCreateServiceError::TableFactoryError(e))?;

        // ファーストクラスコレクションに詰め替え
        let table_columns = TableColumns::new(&table, columns.clone());

        // カラム名の重複チェック
        let no_duplicated_column_name_spec = NoDuplicatedColumnNamesSpecification::new();
        no_duplicated_column_name_spec
            .is_satisfied_by(&table_columns)
            .map_err(|e| TableCreateServiceError::DuplicatedColumnNameError(e))?;

        // テーブルの永続化処理
        let table_id = self
            .table_repository
            .save(&table)
            .await
            .map_err(|e| TableCreateServiceError::TableRepositoryError(e))?;

        // テーブルの永続化が成功した場合、id をセットして OutputData を返す
        table.set_id(table_id);

        let table_with_columns_and_cells =
            TableWithColumnsAndCells::new(&table, columns_with_cells);
        let output_data = TableCreateOutputData::new(table_with_columns_and_cells);
        Ok(output_data)
    }
}

#[cfg(test)]
mod tests {

    use src_domain::models::{
        column::{
            column::Column,
            column_cell::{
                column_cell::ColumnCell, column_cell_id::ColumnCellId,
                column_cell_value::ColumnCellValue,
            },
            column_directory::column_directory_id::ColumnDirectoryId,
            column_name::ColumnName,
        },
        table::table_id::TableId,
    };
    use src_in_memory_infrastructure::{
        column::in_memory_column_repository::InMemoryColumnRepository,
        table::{
            in_memory_table_factory::InMemoryTableFactory,
            in_memory_table_repository::InMemoryTableRepository,
        },
    };

    use super::*;

    #[tokio::test]
    async fn test_handle() -> anyhow::Result<()> {
        let column_repository = InMemoryColumnRepository::new();
        let table_repository = InMemoryTableRepository::new();
        let table_factory = InMemoryTableFactory::new();

        // 事前にセルを4つ作成
        let cell_id1 = ColumnCellId::new("cell_id_1".to_string())?;
        let cell_1 = ColumnCell::new(Some(cell_id1), ColumnCellValue::new(Some(1.0))?);
        let cell_id2 = ColumnCellId::new("cell_id_2".to_string())?;
        let cell_2 = ColumnCell::new(Some(cell_id2), ColumnCellValue::new(Some(2.0))?);
        let cell_id3 = ColumnCellId::new("cell_id_3".to_string())?;
        let cell_3 = ColumnCell::new(Some(cell_id3), ColumnCellValue::new(Some(3.0))?);
        let cell_id4 = ColumnCellId::new("cell_id_4".to_string())?;
        let cell_4 = ColumnCell::new(Some(cell_id4), ColumnCellValue::new(Some(4.0))?);

        column_repository.save_cell(&cell_1).await?;
        column_repository.save_cell(&cell_2).await?;
        column_repository.save_cell(&cell_3).await?;
        column_repository.save_cell(&cell_4).await?;

        // 事前にカラムを2つ作成
        let column_1 = Column::new(
            Some(ColumnId::new("column_id_1".to_string())?),
            ColumnName::new("column_name_1".to_string())?,
            ColumnDirectoryId::new("0".to_string())?,
            vec![cell_1.id().clone(), cell_2.id().clone()],
        );

        let column_2 = Column::new(
            Some(ColumnId::new("column_id_2".to_string()).unwrap()),
            ColumnName::new("column_name_2".to_string()).unwrap(),
            ColumnDirectoryId::new("0".to_string()).unwrap(),
            vec![cell_3.id().clone(), cell_4.id().clone()],
        );

        column_repository.save(&column_1).await.unwrap();
        column_repository.save(&column_2).await.unwrap();

        // サービスのインスタンス化
        let service =
            TableCreateService::new(&column_repository, &table_factory, &table_repository);

        // コマンドの作成
        let command = TableCreateCommand {
            table_name: "table_name_1".to_string(),
            column_ids: vec!["column_id_1".to_string(), "column_id_2".to_string()],
        };

        // テーブル作成サービスの実行
        let result = service.handle(command).await;

        // 結果の整合性を確認
        match result {
            Ok(output_data) => {
                assert_eq!(output_data.table_id, "1");
                assert_eq!(output_data.table_name, "table_name_1");
                assert_eq!(output_data.columns.len(), 2);
                assert_eq!(output_data.columns[0].cells.len(), 2);
                assert_eq!(output_data.columns[1].cells.len(), 2);
            }
            Err(e) => panic!("unexpected error: {:?}", e),
        };

        // テーブルが作成されていることを確認
        let table = table_repository
            .find(&TableId::new("1".to_string()).unwrap())
            .await?;
        match table {
            Some(table) => {
                assert_eq!(table.name().value(), "table_name_1");
                assert_eq!(table.columns().len(), 2);
            }
            None => panic!("table not found"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_handle_with_duplicated_column_name() {
        let column_repository = InMemoryColumnRepository::new();
        let table_repository = InMemoryTableRepository::new();
        let table_factory = InMemoryTableFactory::new();

        // 事前にカラムを2つ作成
        let column_1 = Column::new(
            Some(ColumnId::new("column_id_1".to_string()).unwrap()),
            ColumnName::new("column_name_1".to_string()).unwrap(),
            ColumnDirectoryId::new("0".to_string()).unwrap(),
            vec![],
        );

        let column_2 = Column::new(
            Some(ColumnId::new("column_id_2".to_string()).unwrap()),
            ColumnName::new("column_name_1".to_string()).unwrap(),
            ColumnDirectoryId::new("0".to_string()).unwrap(),
            vec![],
        );

        column_repository.save(&column_1).await.unwrap();
        column_repository.save(&column_2).await.unwrap();

        // サービスのインスタンス化
        let service =
            TableCreateService::new(&column_repository, &table_factory, &table_repository);

        // コマンドの作成
        let command = TableCreateCommand {
            table_name: "table_name_1".to_string(),
            column_ids: vec!["column_id_1".to_string(), "column_id_2".to_string()],
        };

        // テーブル作成サービスの実行
        let result = service.handle(command).await;

        // 結果の整合性を確認
        match result {
            Err(TableCreateServiceError::DuplicatedColumnNameError(_)) => {}
            _ => panic!("unexpected error"),
        }
    }
}
