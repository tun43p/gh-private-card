use std::io;

use crate::features::{github, healthcheck};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use tower_http::services::ServeDir;

/// **Create server routes**
pub fn create_routes() -> Router {
    let serve_dir = get_service(ServeDir::new("./static")).handle_error(handle_error);

    Router::new()
        .nest_service("/ ", serve_dir.clone())
        .fallback_service(serve_dir)
        .route("/github", get(github::routes::get))
        .route("/healthcheck", get(healthcheck::routes::get))
}

/// **Handle server errors**
async fn handle_error(_: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
