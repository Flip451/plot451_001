use src_domain::models::column::column_factory::IColumnFactory;

pub struct SqlxColumnFactory {}

impl SqlxColumnFactory {
    pub fn new() -> Self {
        SqlxColumnFactory {}
    }
}

impl IColumnFactory for SqlxColumnFactory {}