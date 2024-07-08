use std::sync::{Arc, RwLock};

use diesel::SqliteConnection;

#[derive(Clone)]
pub struct DieselTableRepository {
    connection: Arc<RwLock<SqliteConnection>>
}