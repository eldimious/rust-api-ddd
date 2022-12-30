use std::env;
use std::sync::Arc;
use sqlx::migrate::Migrator;
use crate::data::infrastructure::database::{get_db_pool, ping_db};
use crate::data::repositories::user::user_data_store::UserDataStore;
use crate::data::repositories::user::user_repository::UserRepository;
use crate::domain::user::user_service::UserService;
use crate::presentation::http::http_server::{Server, Services};

mod presentation;
mod domain;
mod common;
mod data;

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv()
        .ok()
        .expect("Unable to find .env file. Create one based on the .env.example");
    let port = env::var("PORT").expect("PORT must be set");
    let db_pool = get_db_pool()
        .await
        .expect("Unable to connect to the database");
    ping_db()
        .await
        .expect("Unable to ping the database");
    MIGRATOR
        .run(&db_pool)
        .await
        .expect("Unable to run migrations");

    let user_data_store = UserDataStore::new(db_pool.clone());
    let user_repository = UserRepository::new(user_data_store);
    let user_service = Arc::new(UserService::new(user_repository));

    let server = Server::new(port.parse().unwrap(), Services { user_service });
    server.run().await;

    Ok(())
}