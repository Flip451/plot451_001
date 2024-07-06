use src_domain::{
    models::column::{
        column_directory::column_directory_id::ColumnDirectoryId,
        column_repository::IColumnRepository,
    },
    shared::value_object::ValueObject,
};

use super::{
    column_directory_delete_command::ColumnDirectoryDeleteCommand,
    column_directory_delete_output_data::ColumnDirectoryDeleteOutputData,
    column_directory_delete_service::{
        ColumnDirectoryDeleteServiceError, ColumnDirectoryDeleteServiceResult,
        IColumnDirectoryDeleteService,
    },
};

pub struct ColumnDirectoryDeleteService<'a, CR>
where
    CR: IColumnRepository,
{
    column_repository: &'a CR,
}

impl<'a, CR> ColumnDirectoryDeleteService<'a, CR>
where
    CR: IColumnRepository,
{
    pub fn new(column_repository: &'a CR) -> Self {
        Self { column_repository }
    }
}

impl<'a, CR> IColumnDirectoryDeleteService for ColumnDirectoryDeleteService<'a, CR>
where
    CR: IColumnRepository + Sync,
{
    async fn handle(
        &self,
        command: ColumnDirectoryDeleteCommand,
    ) -> ColumnDirectoryDeleteServiceResult<ColumnDirectoryDeleteOutputData> {
        let ColumnDirectoryDeleteCommand { id } = command;

        let directory_id = ColumnDirectoryId::new(id)
            .map_err(|e| ColumnDirectoryDeleteServiceError::ColumnDirectoryIdError(e))?;

        let directory = self
            .column_repository
            .find_directory(&directory_id)
            .await
            .map_err(|e| ColumnDirectoryDeleteServiceError::ColumnRepositoryError(e))?
            .ok_or(ColumnDirectoryDeleteServiceError::ColumnDirectoryNotFound(
                directory_id,
            ))?;

        self.column_repository
            .delete_directory(directory)
            .await
            .map_err(|e| ColumnDirectoryDeleteServiceError::ColumnRepositoryError(e))?;

        Ok(ColumnDirectoryDeleteOutputData::new())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use src_domain::models::column::{
        column::Column,
        column_cell::{
            column_cell::ColumnCell, column_cell_id::ColumnCellId,
            column_cell_value::ColumnCellValue,
        },
        column_directory::{
            column_directory::ColumnDirectory, column_directory_name::ColumnDirectoryName,
        },
        column_id::ColumnId,
        column_name::ColumnName,
    };
    use src_in_memory_infrastructure::column::in_memory_column_repository::InMemoryColumnRepository;

    use super::*;

    #[tokio::test]
    async fn test_handle() -> anyhow::Result<()> {
        let column_repository = InMemoryColumnRepository::new();

        // 事前にディレクトリを作成しておく
        let directory_id1 = ColumnDirectoryId::new("dir_id1".to_string())?;
        let directory1 = ColumnDirectory::new(
            Some(directory_id1.clone()),
            ColumnDirectoryName::new("dir_name1".to_string())?,
            None,
        );

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
            directory_id1.clone(),
            vec![
                cell1.id().clone(),
                cell2.id().clone(),
            ],
        );

        let column_id2 = ColumnId::new("column_id_2".to_string())?;
        let column2 = Column::new(
            Some(column_id2.clone()),
            ColumnName::new("column_name_2".to_string())?,
            directory_id1.clone(),
            vec![
                cell3.id().clone(),
                cell4.id().clone(),
            ],
        );

        let column_id3 = ColumnId::new("column_id_3".to_string())?;
        let column3 = Column::new(
            Some(column_id3.clone()),
            ColumnName::new("column_name_3".to_string())?,
            directory_id1.clone(),
            vec![
                cell5.id().clone(),
                cell6.id().clone(),
                cell7.id().clone(),
            ],
        );

        // テーブル・カラム・セルをリポジトリに保存
        column_repository.save_directory(&directory1).await?;
        column_repository.save(&column1).await?;
        column_repository.save(&column2).await?;
        column_repository.save(&column3).await?;
        column_repository.save_cell(&cell1).await?;
        column_repository.save_cell(&cell2).await?;
        column_repository.save_cell(&cell3).await?;
        column_repository.save_cell(&cell4).await?;
        column_repository.save_cell(&cell5).await?;
        column_repository.save_cell(&cell6).await?;
        column_repository.save_cell(&cell7).await?;

        let service = ColumnDirectoryDeleteService::new(&column_repository);

        let command = ColumnDirectoryDeleteCommand {
            id: directory1.id().clone_value(),
        };

        let ColumnDirectoryDeleteOutputData {} = service.handle(command).await?;

        // ディレクトリが削除されたことを確認
        assert!(column_repository
            .find_directory(&directory_id1)
            .await?
            .is_none());

        // カラムが削除されたことを確認
        assert!(column_repository
            .find(column1.id(),)
            .await?
            .is_none());
        assert!(column_repository
            .find(column2.id(),)
            .await?
            .is_none());
        assert!(column_repository
            .find(column2.id(),)
            .await?
            .is_none());

        // セルが削除されたことを確認
        assert!(column_repository.find_cell(&cell_id1).await?.is_none());
        assert!(column_repository.find_cell(&cell_id2).await?.is_none());
        assert!(column_repository.find_cell(&cell_id3).await?.is_none());
        assert!(column_repository.find_cell(&cell_id4).await?.is_none());
        assert!(column_repository.find_cell(&cell_id5).await?.is_none());
        assert!(column_repository.find_cell(&cell_id6).await?.is_none());
        assert!(column_repository.find_cell(&cell_id7).await?.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_handle_not_found() -> anyhow::Result<()> {
        let repository = InMemoryColumnRepository::new();
        let service = ColumnDirectoryDeleteService::new(&repository);

        let command = ColumnDirectoryDeleteCommand {
            id: "1".to_string(),
        };

        let result = service.handle(command).await;

        match result {
            Err(ColumnDirectoryDeleteServiceError::ColumnDirectoryNotFound(_)) => Ok(()),
            _ => panic!("unexpected error"),
        }
    }
}
