use src_domain::models::table::table_factory::ITableFactory;

pub struct InMemoryTableFactory {}

impl InMemoryTableFactory {
    pub fn new() -> Self {
        InMemoryTableFactory {}
    }
}

impl ITableFactory for InMemoryTableFactory {}
