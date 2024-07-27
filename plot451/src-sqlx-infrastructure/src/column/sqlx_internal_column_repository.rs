use sqlx::SqliteConnection;
use src_domain::models::column::{
    column::Column,
    column_cell::{column_cell::ColumnCell, column_cell_id::ColumnCellId},
    column_directory::{column_directory::ColumnDirectory, column_directory_id::ColumnDirectoryId},
    column_id::ColumnId,
    column_repository::ColumnRepositoryResult,
};

pub(super) struct InternalColumnRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> InternalColumnRepository<'a> {
    pub(super) fn new(conn: &'a mut SqliteConnection) -> Self {
        InternalColumnRepository { conn }
    }
}

impl<'a> InternalColumnRepository<'a> {
    pub(super) async fn save(&mut self, column: &Column) -> ColumnRepositoryResult<ColumnId> {
        todo!()
    }

    pub(super) async fn find(&mut self, id: &ColumnId) -> ColumnRepositoryResult<Option<Column>> {
        todo!()
    }

    pub(super) async fn find_by_ids(
        &mut self,
        ids: &Vec<ColumnId>,
    ) -> ColumnRepositoryResult<Vec<Column>> {
        todo!()
    }

    pub(super) async fn find_by_directory_id(
        &mut self,
        directory_id: &ColumnDirectoryId,
    ) -> ColumnRepositoryResult<Vec<Column>> {
        todo!()
    }

    pub(super) async fn find_all(&mut self) -> ColumnRepositoryResult<Vec<Column>> {
        todo!()
    }

    pub(super) async fn delete(&mut self, column: Column) -> ColumnRepositoryResult<()> {
        todo!()
    }

    pub(super) async fn save_cell(
        &mut self,
        cell: &ColumnCell,
    ) -> ColumnRepositoryResult<ColumnCellId> {
        todo!()
    }

    pub(super) async fn find_cell(
        &mut self,
        id: &ColumnCellId,
    ) -> ColumnRepositoryResult<Option<ColumnCell>> {
        todo!()
    }

    pub(super) async fn find_cells_by_column_id(
        &mut self,
        column_id: &ColumnId,
    ) -> ColumnRepositoryResult<Vec<ColumnCell>> {
        todo!()
    }

    pub(super) async fn find_cells_by_ids(
        &mut self,
        ids: &Vec<ColumnCellId>,
    ) -> ColumnRepositoryResult<Vec<ColumnCell>> {
        todo!()
    }

    pub(super) async fn delete_cell(&mut self, cell: ColumnCell) -> ColumnRepositoryResult<()> {
        todo!()
    }

    pub(super) async fn save_directory(
        &mut self,
        directory: &ColumnDirectory,
    ) -> ColumnRepositoryResult<ColumnDirectoryId> {
        todo!()
    }

    pub(super) async fn find_directory(
        &mut self,
        id: &ColumnDirectoryId,
    ) -> ColumnRepositoryResult<Option<ColumnDirectory>> {
        todo!()
    }

    pub(super) async fn find_children_directories(
        &mut self,
        parent_id: &ColumnDirectoryId,
    ) -> ColumnRepositoryResult<Vec<ColumnDirectory>> {
        todo!()
    }

    pub(super) async fn delete_directory(
        &mut self,
        directory: ColumnDirectory,
    ) -> ColumnRepositoryResult<()> {
        todo!()
    }
}
