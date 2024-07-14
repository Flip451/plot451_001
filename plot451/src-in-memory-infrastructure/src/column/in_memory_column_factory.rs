use src_domain::models::column::column_factory::IColumnFactory;

pub struct InMemoryColumnFactory {}

impl InMemoryColumnFactory {
    pub fn new() -> Self {
        InMemoryColumnFactory {}
    }
}

impl IColumnFactory for InMemoryColumnFactory {}
