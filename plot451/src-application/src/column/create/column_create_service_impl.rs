use src_domain::{
    models::column::{
        column_cell::column_cell_value::ColumnCellValue,
        column_directory::column_directory_id::ColumnDirectoryId, column_factory::IColumnFactory,
        column_name::ColumnName, column_repository::IColumnRepository, column_with_cells,
    },
    shared::value_object::ValueObject,
};

use super::{
    column_create_command::ColumnCreateCommand,
    column_create_output_data::ColumnCreateOutputData,
    column_create_service::{
        ColumnCreateServiceError, ColumnCreateServiceResult, IColumnCreateService,
    },
};

pub struct ColumnCreateService<'a, 'b, CF, CR>
where
    CF: IColumnFactory,
    CR: IColumnRepository,
{
    column_factory: &'a CF,
    column_repository: &'b CR,
}

impl<'a, 'b, CF, CR> ColumnCreateService<'a, 'b, CF, CR>
where
    CF: IColumnFactory,
    CR: IColumnRepository,
{
    pub fn new(column_factory: &'a CF, column_repository: &'b CR) -> Self {
        ColumnCreateService {
            column_factory,
            column_repository,
        }
    }
}

impl<'a, 'b, CF, CR> IColumnCreateService for ColumnCreateService<'a, 'b, CF, CR>
where
    CF: IColumnFactory + Sync,
    CR: IColumnRepository + Sync,
{
    // TODO: トランザクション処理を追加する
    // TODO: セルの永続化処理を１コマンドで実行するように修正する
    async fn handle(
        &self,
        command: ColumnCreateCommand,
    ) -> ColumnCreateServiceResult<ColumnCreateOutputData> {
        // 値オブジェクトのインスタンス化
        let column_name = ColumnName::new(command.name)
            .map_err(|e| ColumnCreateServiceError::ColumnNameError(e))?;

        let directory_id = ColumnDirectoryId::new(command.directory_id)
            .map_err(|e| ColumnCreateServiceError::ColumnDirectoryIdError(e))?;

        // セルのインスタンス化～永続化
        let mut cells = vec![];
        let mut cell_ids = vec![];
        for cell in command.cells {
            // 値オブジェクトのインスタンス化
            let cell_value = ColumnCellValue::new(cell)
                .map_err(|e| ColumnCreateServiceError::ColumnCellValueError(e))?;
            let mut cell = self
                .column_factory
                .create_cell(cell_value)
                .await
                .map_err(|e| ColumnCreateServiceError::ColumnFactoryError(e))?;

            // セルの永続化
            let cell_id = self
                .column_repository
                .save_cell(&cell)
                .await
                .map_err(|e| ColumnCreateServiceError::ColumnRepositoryError(e))?;
            cell.set_id(cell_id);

            // セルの永続化に成功した場合、リストに追加
            cell_ids.push(cell.id().clone());
            cells.push(cell);
        }

        // カラムのインスタンス化
        let mut column = self
            .column_factory
            .create_column(column_name, directory_id, cell_ids)
            .await
            .map_err(|e| ColumnCreateServiceError::ColumnFactoryError(e))?;

        // カラムの永続化
        let column_id = self
            .column_repository
            .save(&column)
            .await
            .map_err(|e| ColumnCreateServiceError::ColumnRepositoryError(e))?;

        // カラムの永続化が成功した場合、id をセットして OutputData を返す
        column.set_id(column_id);

        // ファーストクラスコレクションに詰め替え
        let column_with_cells = column_with_cells::ColumnWithCells::new(&column, cells);

        let output_data = ColumnCreateOutputData::new(column_with_cells);
        Ok(output_data)
    }
}

#[cfg(test)]
mod tests {
    use src_domain::models::column::column_id::ColumnId;
    use src_in_memory_infrastructure::column::{
        in_memory_column_factory::InMemoryColumnFactory,
        in_memory_column_repository::InMemoryColumnRepository,
    };

    use super::*;

    #[tokio::test]
    async fn test_handle() {
        let column_factory = InMemoryColumnFactory::new();
        let column_repository = InMemoryColumnRepository::new();

        // サービスのインスタンス化
        let service = ColumnCreateService::new(&column_factory, &column_repository);

        // コマンドの作成
        let command = ColumnCreateCommand {
            name: "test_column".to_string(),
            directory_id: "0".to_string(),
            cells: vec![Some(1.), Some(2.), Some(3.), None, Some(5.)],
        };

        // カラム作成サービスの実行
        let result = service.handle(command).await;

        // サービスの実行結果の検証
        match result {
            Ok(output_data) => {
                assert_eq!(output_data.column_id, "1");
                assert_eq!(output_data.column_name, "test_column");
                assert_eq!(output_data.cells.len(), 5);
            }
            Err(e) => {
                panic!("Unexpected error: {:?}", e);
            }
        }

        // カラムが作成されていることを確認
        let column = column_repository
            .find(&ColumnId::new("1".to_string()).unwrap())
            .await
            .unwrap();
        match column {
            Some(column) => {
                assert_eq!(column.name().clone_value(), "test_column");
                assert_eq!(column.cells().len(), 5);
            }
            None => {
                panic!("Column not found");
            }
        }
    }
}
