mod create_task;
mod get_task;
mod partial_update_tasks;
mod update_task;

use axum::{
    http::Method,
    routing::{get, patch, post, put},
    Extension, Router,
};
use create_task::create_task;
use get_task::{get_all_tasks, get_one_task};
use partial_update_tasks::partial_update;
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};
use update_task::atomic_update;

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
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/:id", get(get_one_task))
        .route("/tasks/:id", put(atomic_update))
        .route("/tasks/:id", patch(partial_update))
        .layer(cors)
        .layer(Extension(database))
}
