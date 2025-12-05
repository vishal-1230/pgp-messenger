use sqlx::{MySql, Pool};
use sqlx::pool::PoolOptions;
use std::time::Duration;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<MySql>,
}

impl Database {
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        println!("Connecting to database at {database_url}...");
        let pool = PoolOptions::<MySql>::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(5))
            .connect(database_url)
            .await?;
        println!("Database connected successfully.");
        Ok(Database { pool })
    }
}