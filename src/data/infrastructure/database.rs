use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sqlx::pool::Pool;
use sqlx::postgres::{PgPoolOptions, Postgres};
use std::env;

use crate::common::error::Error;
use crate::common::error::Result;

const DB_POOL_MAX_CONNECTIONS: u32 = 5;

pub type DbPool = Pool<Postgres>;

lazy_static! {
    static ref DB_POOL: AsyncOnce<Result<DbPool>> = AsyncOnce::new(async { create_pool().await });
}

pub async fn create_pool() -> Result<Pool<Postgres>> {
    let db_uri = env::var("DATABASE_URL").expect("Missing \"DATABASE_URL\" environment variable");

    PgPoolOptions::new()
        .max_connections(DB_POOL_MAX_CONNECTIONS)
        .connect(&db_uri)
        .await
        .map_err(Error::from)
}

pub async fn get_db_pool() -> Result<DbPool> {
    DB_POOL.get().await.clone()
}

pub async fn ping_db() -> Result<()> {
    println!("Checking on database connection...");
    let pool = get_db_pool().await?;

    sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await
        .expect("Failed to PING database");
    println!("Database PING executed successfully!");

    Ok(())
}