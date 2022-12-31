use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sqlx::pool::Pool;
use sqlx::postgres::{PgPoolOptions, Postgres};
use std::env;

use crate::common::error::CustomError;
use crate::common::error::Result;
use crate::Configuration;

const DB_POOL_MAX_CONNECTIONS: u32 = 5;

pub type DbPool = Pool<Postgres>;

lazy_static! {
    static ref DB_POOL: AsyncOnce<Result<DbPool>> = AsyncOnce::new(async { create_connection().await });
}

pub async fn create_connection() -> Result<Pool<Postgres>> {
    let config = Configuration::new();
    let db_uri = config.database.uri;

    PgPoolOptions::new()
        .max_connections(DB_POOL_MAX_CONNECTIONS)
        .connect(&db_uri)
        .await
        .map_err(CustomError::from)
}

pub async fn get_db_connection() -> Result<DbPool> {
    DB_POOL.get().await.clone()
}

pub async fn check_db_connection
() -> Result<()> {
    println!("Checking on database connection...");
    let pool = get_db_connection().await?;

    sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await
        .expect("Failed to PING database");
    println!("Database PING executed successfully!");

    Ok(())
}