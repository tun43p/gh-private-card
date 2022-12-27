use super::{
    helpers::{create_github_client, create_github_repository_card},
    models::Repository,
};
use axum::{extract::Query, response::Html};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    url: String,
}

/// **Get GitHub repository or pull request card by requesting the GitHub API.**
///
/// Example: `curl -X GET http://localhost:3000/github?url=https://github.com/user/repo`
pub async fn get(Query(params): Query<Params>) -> Html<String> {
    let github_client = create_github_client();

    // TODO(tun43p): Check if is a repository or a pull request

    let repository_url = params.url.replace("github.com/", "api.github.com/repos/");

    let repository_result = github_client
        .get(repository_url)
        .send()
        .await
        .expect("error getting response");

    let repository: Repository = repository_result
        .json()
        .await
        .expect("error getting repository");

    // TODO(tun43p): Return an image
    Html(create_github_repository_card(&repository))
}
