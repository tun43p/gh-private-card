use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new().route("/", get(get_repository));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_repository(Json(payload): Json<Repository>) -> impl IntoResponse {
    // TODO(tun43p): Improve parsing
    let url = payload
        .url
        .replace("https://github.com/", "https://api.github.com/repos");

    // TODO(tun43p): Use `headers` and not multiples `header`
    let res = reqwest::Client::new()
        .get(url)
        .header("Accept", "application/vnd.github+json")
        .header(
            "Authorization",
            format!("Bearer {}", env::var("GITHUB_TOKEN").unwrap()),
        )
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await;

    match res {
        Ok(response) => (StatusCode::OK, Json(payload)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)),
    }
}

#[derive(Deserialize, Serialize)]
struct Repository {
    url: String,
}
