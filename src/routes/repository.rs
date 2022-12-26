use crate::{
    helpers::{client, html::create_github_repository_card},
    models::repository::Repository,
};
use axum::{extract::Query, response::Html};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    url: String,
}

/// **Get repository informations from GitHub API.**
///
/// Example: `curl -X GET http://localhost:3000/repository?url=https://github.com/user/repo`
pub async fn get_repository_card(Query(params): Query<Params>) -> Html<String> {
    let github_client = client::create_github_client();

    let url = params.url.replace("github.com/", "api.github.com/repos/");

    let result = github_client
        .get(url)
        .send()
        .await
        .expect("error getting response");

    let repository: Repository = result.json().await.expect("error getting repository");

    Html(create_github_repository_card(&repository))
}
