// 参考：https://docs.diesel.rs/master/diesel/r2d2/index.html

use diesel::{r2d2::{ConnectionManager, Pool}, SqliteConnection};

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection(database_url: &str) -> SqlitePool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}