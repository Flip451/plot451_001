// https://gihyo.jp/article/2022/10/rust-monthly-topics-02

use std::str::FromStr;

use anyhow::Result;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};

// SQLite のコネクションプールを作成して返す
pub async fn create_sqlite_pool(database_url: &str) -> Result<SqlitePool> {
    // コネクションの設定
    let connection_options = SqliteConnectOptions::from_str(&database_url)?
        // DB が存在しないなら作成する
        .create_if_missing(true)
        // トランザクション使用時の性能向上のため、WAL モードを有効にする
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    // 上の設定を使ってコネクションを作成
    let pool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await?;
    Ok(pool)
}

// マイグレーションを実行する
pub async fn migrate(pool: &SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

#[cfg(test)]
pub async fn create_and_migrate_test_pool() -> Result<SqlitePool> {
    let pool = create_sqlite_pool("sqlite::memory:").await?;
    migrate(&pool).await?;
    Ok(pool)
}
