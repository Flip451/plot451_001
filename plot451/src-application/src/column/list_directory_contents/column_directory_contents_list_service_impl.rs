use src_domain::{
    models::column::{
        column_directory::{
            column_directory_id::ColumnDirectoryId, directory_contents::DirectoryContents,
        },
        column_repository::IColumnRepository,
    },
    shared::value_object::ValueObject,
};

use super::{
    column_directory_contents_list_command::ColumnDirectoryContentsListCommand,
    column_directory_contents_list_output_data::ColumnDirectoryContentsListOutputData,
    column_directory_contents_list_service::{
        ColumnDirectoryContentsListServiceError, ColumnDirectoryContentsListServiceResult,
        IColumnDirectoryContentsListService,
    },
};

pub struct ColumnDirectoryContentsListService<'a, CR>
where
    CR: IColumnRepository,
{
    column_repository: &'a CR,
}

impl<'a, CR> ColumnDirectoryContentsListService<'a, CR>
where
    CR: IColumnRepository,
{
    pub fn new(column_repository: &'a CR) -> Self {
        Self { column_repository }
    }
}

impl<'a, CR> IColumnDirectoryContentsListService for ColumnDirectoryContentsListService<'a, CR>
where
    CR: IColumnRepository + Sync,
{
    async fn handle(
        &self,
        command: ColumnDirectoryContentsListCommand,
    ) -> ColumnDirectoryContentsListServiceResult<ColumnDirectoryContentsListOutputData> {
        // コマンドオブジェクトからディレクトリIDを取得
        let ColumnDirectoryContentsListCommand { directory_id } = command;
        let directory_id = ColumnDirectoryId::new(directory_id)
            .map_err(|e| ColumnDirectoryContentsListServiceError::ColumnDirectoryIdError(e))?;

        // ディレクトリの存在を確認
        let directory = self
            .column_repository
            .find_directory(&directory_id)
            .await
            .map_err(|e| ColumnDirectoryContentsListServiceError::ColumnRepositoryError(e))?;

        let directory = match directory {
            Some(d) => d,
            None => {
                return Err(
                    ColumnDirectoryContentsListServiceError::ColumnDirectoryNotFound(directory_id),
                );
            }
        };

        // ディレクトリ内のカラムを取得
        let columns = self
            .column_repository
            .find_by_directory_id(&directory_id)
            .await
            .map_err(|e| ColumnDirectoryContentsListServiceError::ColumnRepositoryError(e))?;

        let directories = self
            .column_repository
            .find_children_directories(&directory_id)
            .await
            .map_err(|e| ColumnDirectoryContentsListServiceError::ColumnRepositoryError(e))?;

        // ファーストクラスコレクションにデータを詰め替え
        let directory_columns = DirectoryContents::new(&directory, columns, directories);

        // DTOにデータを詰め替え
        let output_data = ColumnDirectoryContentsListOutputData::new(directory_columns);

        Ok(output_data)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    use crate::column::list_directory_contents::column_directory_contents_list_output_data::{
        ColumnInOutputData, DirectoryInOutputData,
    };
    use src_domain::{
        models::column::{
            column::Column,
            column_directory::{
                column_directory::ColumnDirectory, column_directory_id::ColumnDirectoryId,
                column_directory_name::ColumnDirectoryName,
            },
            column_id::ColumnId,
            column_name::ColumnName,
            column_repository::IColumnRepository,
        },
        shared::value_object::ValueObject,
    };
    use src_in_memory_infrastructure::column::in_memory_column_repository::InMemoryColumnRepository;

    #[tokio::test]
    async fn test_column_list_service() -> anyhow::Result<()> {
        let column_repository = InMemoryColumnRepository::new();

        /* データ構造
         * directory1--+--column1
         *             +--column2
         *             +--column3
         *             +--directory2
         *             +--directory3
         */
        // 事前にデータを登録
        let directory_id1 = ColumnDirectoryId::new("1".to_string())?;
        let directory1 = ColumnDirectory::new(
            Some(directory_id1.clone()),
            ColumnDirectoryName::new("directory1".to_string())?,
            None,
        );

        let directory_id2 = ColumnDirectoryId::new("2".to_string())?;
        let directory2 = ColumnDirectory::new(
            Some(directory_id2),
            ColumnDirectoryName::new("directory2".to_string())?,
            Some(directory_id1.clone()),
        );

        let directory_id3 = ColumnDirectoryId::new("3".to_string())?;
        let directory3 = ColumnDirectory::new(
            Some(directory_id3),
            ColumnDirectoryName::new("directory3".to_string())?,
            Some(directory_id1.clone()),
        );

        let column_id1 = ColumnId::new("1".to_string())?;
        let column1 = Column::new(
            Some(column_id1.clone()),
            ColumnName::new("column1".to_string())?,
            directory_id1.clone(),
            vec![],
        );

        let column_id2 = ColumnId::new("2".to_string())?;
        let column2 = Column::new(
            Some(column_id2.clone()),
            ColumnName::new("column2".to_string())?,
            directory_id1.clone(),
            vec![],
        );

        let column_id3 = ColumnId::new("3".to_string())?;
        let column3 = Column::new(
            Some(column_id3.clone()),
            ColumnName::new("column3".to_string())?,
            directory_id1.clone(),
            vec![],
        );

        column_repository.save_directory(&directory1).await?;
        column_repository.save_directory(&directory2).await?;
        column_repository.save_directory(&directory3).await?;
        column_repository.save(&column1).await?;
        column_repository.save(&column2).await?;
        column_repository.save(&column3).await?;

        // サービスのインスタンス化
        let service = ColumnDirectoryContentsListService::new(&column_repository);

        // コマンドを作成
        let command = ColumnDirectoryContentsListCommand {
            directory_id: "1".to_string(),
        };

        // サービスの実行
        let ColumnDirectoryContentsListOutputData {
            columns,
            directories,
        } = service.handle(command).await?;

        assert_eq!(
            HashSet::from_iter(columns.iter().cloned()),
            HashSet::from([
                ColumnInOutputData {
                    column_id: "1".to_string(),
                    column_name: "column1".to_string()
                },
                ColumnInOutputData {
                    column_id: "2".to_string(),
                    column_name: "column2".to_string()
                },
                ColumnInOutputData {
                    column_id: "3".to_string(),
                    column_name: "column3".to_string()
                }
            ])
        );

        assert_eq!(
            HashSet::from_iter(directories.iter().cloned()),
            HashSet::from([
                DirectoryInOutputData {
                    directory_id: "2".to_string(),
                    directory_name: "directory2".to_string()
                },
                DirectoryInOutputData {
                    directory_id: "3".to_string(),
                    directory_name: "directory3".to_string()
                }
            ])
        );
        Ok(())
    }
}
