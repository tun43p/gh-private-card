use crate::{helpers::client, models};
use axum::{extract::Query, response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    url: String,
}

/// **Get repository informations from GitHub API.**
///
/// Example: `curl -X GET http://localhost:3000/repository?url=https://github.com/user/repo`
pub async fn get_repository(Query(params): Query<Params>) -> impl IntoResponse {
    let client = client::create_client();

    let url = params.url.replace("github.com/", "api.github.com/repos/");

    let result = client
        .get(url)
        .send()
        .await
        .expect("error getting response");

    let repository: models::repository::Repository =
        result.json().await.expect("error getting repository");

    (StatusCode::OK, Json(repository))
}
