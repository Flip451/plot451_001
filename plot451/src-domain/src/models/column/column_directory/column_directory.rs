use thiserror::Error;

use crate::shared::entity::Entity;

use super::{column_directory_id::ColumnDirectoryId, column_directory_name::ColumnDirectoryName};

#[derive(Debug, Clone, Hash, Eq)]
pub struct ColumnDirectory {
    id: Option<ColumnDirectoryId>,
    name: ColumnDirectoryName,
    parent: Option<ColumnDirectoryId>,
}

impl ColumnDirectory {
    pub fn new(id: Option<ColumnDirectoryId>, name: ColumnDirectoryName, parent: Option<ColumnDirectoryId>) -> Self {
        Self { id, name, parent }
    }

    // getter & setter
    pub fn id(&self) -> &Option<ColumnDirectoryId> {
        &self.id
    }

    pub fn set_id(&mut self, id: ColumnDirectoryId) {
        if let Some(_) = self.id {
            panic!("id cannot be change");
        }
        self.id = Some(id);
    }

    pub fn name(&self) -> &ColumnDirectoryName {
        &self.name
    }

    pub fn parent(&self) -> &Option<ColumnDirectoryId> {
        &self.parent
    }

    // ディレクトリ名の変更
    pub fn change_name(&mut self, name: ColumnDirectoryName) {
        self.name = name;
    }

    // ディレクトリの移動
    pub fn move_to(&mut self, new_parent: Option<ColumnDirectoryId>) {
        self.parent = new_parent;
    }
}

#[derive(Debug, Error)]
enum ColumnDirectoryError {}

impl Entity for ColumnDirectory {
    type Identity = ColumnDirectoryId;

    fn identity(&self) -> &Self::Identity {
        self.id.as_ref().unwrap()
    }
}

impl PartialEq for ColumnDirectory {
    fn eq(&self, other: &Self) -> bool {
        Entity::eq(self, other)
    }
}