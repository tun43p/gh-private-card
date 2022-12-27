use crate::features::{github, healthcheck};
use axum::{routing::get, Router};

/// **Create server routes**
pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(healthcheck::routes::get))
        .route("/github", get(github::routes::get))
}
