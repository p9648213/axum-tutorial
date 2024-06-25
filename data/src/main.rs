use data::run;
use dotenvy::dotenv;
use log::info;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to read .env file");

    let database_uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    if database_uri.is_empty() {
        panic!("DATABASE_URL is empty");
    }

    info!("Database URL: {}", database_uri);

    run(database_uri.as_str()).await;
}
