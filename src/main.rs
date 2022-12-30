use std::env;
use crate::presentation::http::http_server::Server;

mod presentation;
mod domain;
mod common;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv()
        .ok()
        .expect("Unable to find .env file. Create one based on the .env.example");
    let port = env::var("PORT").expect("PORT must be set");
    let server = Server::new(port.parse().unwrap());
    server.run().await;
    Ok(())
}