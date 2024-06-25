mod database;
mod routes;
use routes::create_routes;
use sea_orm::Database;

pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await.unwrap();

    let app = create_routes(database);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
