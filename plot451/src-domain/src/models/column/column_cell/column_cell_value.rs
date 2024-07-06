use crate::shared::value_object::ValueObject;
use thiserror::Error;

pub type CellRawValue = Option<f64>;

// value object
#[derive(Debug, Clone)]
pub struct ColumnCellValue {
    value: CellRawValue,
}

#[derive(Debug, Error)]
pub enum ColumnCellValueError {
    #[error("ColumnCellValueParseError: [{0}]")]
    ParseError(String),
}

impl ValueObject for ColumnCellValue {
    type Value = CellRawValue;

    type Error = ColumnCellValueError;

    fn new(value: Self::Value) -> Result<Self, Self::Error> {
        Ok(Self { value })
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }

    fn clone_value(&self) -> Self::Value {
        self.value.clone()
    }
}

impl ColumnCellValue {
    fn parse(str: &str) -> Result<Self, ColumnCellValueError> {
        let str = str.trim();
        if str.is_empty() {
            return Ok(Self { value: None });
        }
        let value = str
            .parse::<f64>()
            .map_err(|e| ColumnCellValueError::ParseError(e.to_string()))?;
        Ok(Self { value: Some(value) })
    }
}

impl PartialEq for ColumnCellValue {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let cell_value = ColumnCellValue::parse(" 1.0　").unwrap();
        assert_eq!(cell_value.value(), &Some(1.0));
    }

    #[test]
    fn test_parse_empty() {
        let cell_value = ColumnCellValue::parse("").unwrap();
        assert_eq!(cell_value.value(), &None);
    }

    #[test]
    fn test_parse_whitespace() {
        let cell_value = ColumnCellValue::parse(" ").unwrap();
        assert_eq!(cell_value.value(), &None);
    }

    #[test]
    fn test_parse_double_byte_whitespace() {
        // 全角スペースの場合
        let cell_value = ColumnCellValue::parse("　").unwrap();
        assert_eq!(cell_value.value(), &None);
    }

    #[test]
    fn test_parse_error() {
        let cell_value = ColumnCellValue::parse("a");
        assert!(cell_value.is_err());
    }
}
