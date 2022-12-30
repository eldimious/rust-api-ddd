#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv()
        .ok()
        .expect("Unable to find .env file. Create one based on the .env.example");

    Ok(())
}