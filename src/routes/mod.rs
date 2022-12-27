mod github;
mod healthcheck;

use self::{github::get_github_card, healthcheck::get_healthcheck};
use axum::{routing::get, Router};

/// **Create server routes**
pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(get_healthcheck))
        .route("/repository", get(get_github_card))
}
