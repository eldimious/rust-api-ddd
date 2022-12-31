use std::env;
use std::sync::Arc;
use sqlx::migrate::Migrator;
use crate::configuration::configuration::Configuration;
use crate::data::infrastructure::database::{check_db_connection, get_db_connection};
use crate::data::repositories::user::user_data_store::UserDataStore;
use crate::data::repositories::user::user_repository::UserRepository;
use crate::domain::user::user_service::UserService;
use crate::presentation::http::http_server::{Server, Services};

mod presentation;
mod domain;
mod common;
mod data;
mod configuration;

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = Configuration::new();
    let db_pool = get_db_connection()
        .await
        .expect("Unable to connect to the database");
    check_db_connection()
        .await
        .expect("Unable to ping the database");
    MIGRATOR
        .run(&db_pool)
        .await
        .expect("Unable to run migrations");

    let user_data_store = UserDataStore::new(db_pool.clone());
    let user_repository = UserRepository::new(user_data_store);
    let user_service = Arc::new(UserService::new(user_repository));

    let server = Server::new(config.server.port.parse().unwrap(), Services { user_service });
    server.run().await;

    Ok(())
}