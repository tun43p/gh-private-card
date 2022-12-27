use axum::{routing::get, Router};

use crate::features::{github::routes::get_github_card, healthcheck::get_healthcheck};

/// **Create server routes**
pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(get_healthcheck))
        .route("/github", get(get_github_card))
}
