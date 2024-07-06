use thiserror::Error;

use std::collections::HashSet;

use crate::{
    models::column::column_name::ColumnName, shared::specification::Specification,
};

use super::table_columns::TableColumns;

pub struct NoDuplicatedColumnNamesSpecification {}

impl NoDuplicatedColumnNamesSpecification {
    pub fn new() -> Self {
        Self {}
    }
}

impl Specification for NoDuplicatedColumnNamesSpecification {
    type T = TableColumns;
    type Error = NoDuplicateColumnNameSpecificationError;

    fn is_satisfied_by(
        &self,
        table_columns: &TableColumns,
    ) -> Result<(), NoDuplicateColumnNameSpecificationError> {
        let column_names = table_columns.column_names();
        let mut set = HashSet::new();
        for column_name in column_names {
            if !set.insert(column_name) {
                return Err(
                    NoDuplicateColumnNameSpecificationError::DuplicatedColumnNames(
                        column_name.clone(),
                    ),
                );
            }
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum NoDuplicateColumnNameSpecificationError {
    #[error("column names are duplicated, duplicated column names: {0}")]
    DuplicatedColumnNames(ColumnName),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::column::column::Column;
    use crate::models::column::column_directory::column_directory_id::ColumnDirectoryId;
    use crate::models::column::column_id::ColumnId;
    use crate::models::column::column_name::ColumnName;
    use crate::models::table::table_columns::TableColumns;
    use crate::shared::value_object::ValueObject;

    #[test]
    fn test_is_satisfied() {
        let table_columns = TableColumns::new(vec![
            Column::new(
                Some(ColumnId::new("column_id_1".to_string()).unwrap()),
                ColumnName::new("column1".to_string()).unwrap(),
                ColumnDirectoryId::new("0".to_string()).unwrap(),
                vec![],
            ),
            Column::new(
                Some(ColumnId::new("column_id_2".to_string()).unwrap()),
                ColumnName::new("column2".to_string()).unwrap(),
                ColumnDirectoryId::new("0".to_string()).unwrap(),
                vec![],
            ),
            Column::new(
                Some(ColumnId::new("column_id_3".to_string()).unwrap()),
                ColumnName::new("column3".to_string()).unwrap(),
                ColumnDirectoryId::new("0".to_string()).unwrap(),
                vec![],
            ),
        ]);
        let specification = NoDuplicatedColumnNamesSpecification::new();
        assert!(specification.is_satisfied_by(&table_columns).is_ok());
    }

    #[test]
    fn test_is_not_satisfied() {
        let table_columns = TableColumns::new(vec![
            Column::new(
                Some(ColumnId::new("column_id_1".to_string()).unwrap()),
                ColumnName::new("column1".to_string()).unwrap(),
                ColumnDirectoryId::new("0".to_string()).unwrap(),
                vec![],
            ),
            Column::new(
                Some(ColumnId::new("column_id_2".to_string()).unwrap()),
                ColumnName::new("column2".to_string()).unwrap(),
                ColumnDirectoryId::new("0".to_string()).unwrap(),
                vec![],
            ),
            Column::new(
                Some(ColumnId::new("column_id_3".to_string()).unwrap()),
                ColumnName::new("column1".to_string()).unwrap(),
                ColumnDirectoryId::new("0".to_string()).unwrap(),
                vec![],
            ),
        ]);
        let specification = NoDuplicatedColumnNamesSpecification::new();
        assert!(specification.is_satisfied_by(&table_columns).is_err());
    }
}
