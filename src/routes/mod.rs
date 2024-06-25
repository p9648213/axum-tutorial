mod always_errors;
mod custom_json_extractor;
mod get_json;
mod hello_world;
mod middleware_message;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_user_agents;
mod path_variable;
mod query_params;
mod read_middleware_custom_header;
mod return_201;
mod set_middleware_custom_header;
mod validate_with_serde;

use always_errors::always_errors;
use axum::{
    http::Method,
    middleware,
    routing::{get, patch, post},
    Extension, Router,
};
use custom_json_extractor::custom_json_extractor;
use get_json::get_json;
use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_user_agents::mirror_user_agents;
use path_variable::{hard_coded_path, path_variable};
use query_params::query_params;
use read_middleware_custom_header::read_middleware_custom_header;
use return_201::return_201;
use set_middleware_custom_header::set_middleware_custom_header;
use tower_http::cors::{Any, CorsLayer};
use validate_with_serde::validate_with_serde;

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from share data".to_owned(),
    };

    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", patch(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/:id", get(path_variable))
        .route("/path_variable/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agents", get(mirror_user_agents))
        .route("/middleware_message", get(middleware_message))
        .layer(cors)
        .layer(Extension(shared_data))
        .route("/always_errors", get(always_errors))
        .route("/return_201", post(return_201))
        .route("/get_json", get(get_json))
        .route("/validate_data", post(validate_with_serde))
        .route("/custom_json_extractor", post(custom_json_extractor))
}
