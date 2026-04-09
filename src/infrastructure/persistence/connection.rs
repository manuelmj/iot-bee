use sqlx::sqlite::{SqlitePool as SqlxPool, SqlitePoolOptions};
use sqlx::Error as SqlxError;
use std::time::Duration;

#[derive(Clone)]
pub struct SqliteDb {
    pool: SqlxPool,
}

impl SqliteDb {
    pub async fn new(db_url: &str) -> Result<Self, SqlxError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(300))
            .max_lifetime(Duration::from_secs(1800))
            .connect(db_url)
            .await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlxPool {
        &self.pool
    }
}