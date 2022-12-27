mod healthcheck;
mod repository;

use self::{healthcheck::get_healthcheck, repository::get_repository_card};
use axum::{routing::get, Router};

/// **Create server routes**
pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(get_healthcheck))
        .route("/repository", get(get_repository_card))
}
