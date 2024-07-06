use src_domain::{
    models::column::{
        column_directory::{
            column_directory_id::ColumnDirectoryId, column_directory_name::ColumnDirectoryName,
        },
        column_factory::IColumnFactory,
        column_repository::IColumnRepository,
    },
    shared::value_object::ValueObject,
};

use super::{
    column_directory_create_command::ColumnDirectoryCreateCommand,
    column_directory_create_output_data::ColumnDirectoryCreateOutputData,
    column_directory_create_service::{
        ColumnDirectoryCreateServiceError, ColumnDirectoryCreateServiceResult,
        IColumnDirectoryCreateService,
    },
};

pub struct ColumnDirectoryCreateService<'a, 'b, CF, CR>
where
    CF: IColumnFactory,
    CR: IColumnRepository,
{
    column_factory: &'a CF,
    column_repository: &'b CR,
}

impl<'a, 'b, CF, CR> ColumnDirectoryCreateService<'a, 'b, CF, CR>
where
    CF: IColumnFactory,
    CR: IColumnRepository,
{
    pub fn new(column_factory: &'a CF, column_repository: &'b CR) -> Self {
        Self {
            column_factory,
            column_repository,
        }
    }
}

impl<'a, 'b, CF, CR> IColumnDirectoryCreateService for ColumnDirectoryCreateService<'a, 'b, CF, CR>
where
    CF: IColumnFactory,
    CR: IColumnRepository,
{
    // TODO: トランザクション処理を追加する
    async fn handle(
        &self,
        command: ColumnDirectoryCreateCommand,
    ) -> ColumnDirectoryCreateServiceResult<ColumnDirectoryCreateOutputData> {
        let ColumnDirectoryCreateCommand { name, parent_id } = command;

        // 値オブジェクトのインスタンス化
        let name = ColumnDirectoryName::new(name)
            .map_err(|e| ColumnDirectoryCreateServiceError::ColumnDirectoryNameError(e))?;

        let parent_id = match parent_id {
            Some(parent_id) => {
                let parent_id = ColumnDirectoryId::new(parent_id)
                    .map_err(|e| ColumnDirectoryCreateServiceError::ColumnDirectoryIdError(e))?;
                let found_parent_directory = self
                    .column_repository
                    .find_directory(&parent_id)
                    .await
                    .map_err(|e| ColumnDirectoryCreateServiceError::ColumnRepositoryError(e))?;
                if let None = found_parent_directory {
                    return Err(ColumnDirectoryCreateServiceError::ColumnDirectoryNotFound(
                        parent_id,
                    ));
                }
                Some(parent_id)
            }
            None => None,
        };

        // エンティティのインスタンス化
        let mut directory = self
            .column_factory
            .create_directory(name, parent_id)
            .await
            .map_err(|e| ColumnDirectoryCreateServiceError::ColumnDirectoryFactoryError(e))?;

        // エンティティの永続化
        let directory_id = self
            .column_repository
            .save_directory(&directory)
            .await
            .map_err(|e| ColumnDirectoryCreateServiceError::ColumnRepositoryError(e))?;

        // エンティティの永続化が成功した場合、id をセットして OutputData を返す
        directory.set_id(directory_id);
        let output_data = ColumnDirectoryCreateOutputData::new(directory);
        Ok(output_data)
    }
}

#[cfg(test)]
mod tests {
    use src_in_memory_infrastructure::column::{
        in_memory_column_factory::InMemoryColumnFactory,
        in_memory_column_repository::InMemoryColumnRepository,
    };

    use super::*;

    #[tokio::test]
    async fn test_handle() -> anyhow::Result<()> {
        let column_factory = InMemoryColumnFactory::new();
        let column_repository = InMemoryColumnRepository::new();

        // サービスのインスタンス化
        let service = ColumnDirectoryCreateService::new(&column_factory, &column_repository);

        let command = ColumnDirectoryCreateCommand {
            name: "directory_name1".to_string(),
            parent_id: None,
        };

        let ColumnDirectoryCreateOutputData {
            directory_id,
            directory_name,
            parent_id,
        } = service.handle(command).await?;

        assert_eq!(directory_id, "1".to_string());
        assert_eq!(directory_name, "directory_name1".to_string());
        assert_eq!(parent_id, None);

        let command = ColumnDirectoryCreateCommand {
            name: "directory_name2".to_string(),
            parent_id: Some("1".to_string()),
        };

        let ColumnDirectoryCreateOutputData {
            directory_id,
            directory_name,
            parent_id,
        } = service.handle(command).await?;

        assert_eq!(directory_id, "2".to_string());
        assert_eq!(directory_name, "directory_name2".to_string());
        assert_eq!(parent_id, Some("1".to_string()));

        Ok(())
    }
}
