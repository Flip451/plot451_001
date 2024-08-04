use src_domain::models::table::table_factory::ITableFactory;

pub struct SqlxTableFactory {}

impl SqlxTableFactory {
    pub fn new() -> Self {
        SqlxTableFactory {}
    }
}

impl ITableFactory for SqlxTableFactory {}
