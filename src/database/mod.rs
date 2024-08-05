use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Error, Pool, Sqlite, SqlitePool};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn init_pool() -> Result<Pool<Sqlite>, Error> {
    connect_database().await
}

pub async fn reinitialize_pool(pool: &Arc<Mutex<SqlitePool>>) -> Result<(), Error> {
    let new_pool = init_pool().await?;
    let mut pool_guard = pool.lock().await;
    *pool_guard = new_pool;
    Ok(())
}
pub async fn close_pool(pool: &Arc<Mutex<SqlitePool>>) -> Result<(), Error> {
    let pool_guard = pool.lock().await;
    pool_guard.close().await;
    Ok(())
}

pub async fn connect_database() -> Result<Pool<Sqlite>, Error> {
    let options: SqliteConnectOptions =
        SqliteConnectOptions::from_str("sqlite://logDB.db")?.create_if_missing(true);

    let pool: Pool<Sqlite> = SqlitePool::connect_with(options).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            entity TEXT,
            log TEXT
        );",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
