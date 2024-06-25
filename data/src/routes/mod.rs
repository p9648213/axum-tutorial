mod create_task;

use axum::{
    http::Method,
    routing::{get, post},
    Extension, Router,
};
use create_task::create_task;
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

async fn pong() -> String {
    "pong".to_owned()
}

pub fn create_routes(database: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/ping", get(pong))
        .route("/tasks", post(create_task))
        .layer(cors)
        .layer(Extension(database))
}
