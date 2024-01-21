use sqlx::{Error, SqlitePool};

pub async fn get_database_connection_pool(url: &str) -> Result<sqlx::SqlitePool, Error> {
    return SqlitePool::connect(url).await;
}
