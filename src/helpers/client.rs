use super::env;
use reqwest::{header::HeaderMap, header::HeaderValue, Client};

/// **Create a client for making requests on the GitHub API.**
pub fn create_github_client() -> Client {
    let mut headers = HeaderMap::new();

    headers.insert("User-Agent", header_value("*"));
    headers.insert("Accept", header_value("application/vnd.github+json"));
    headers.insert(
        "Authorization",
        header_value(format!("Bearer {}", env::get_github_token()).as_str()),
    );
    headers.insert("X-GitHub-Api-Version", header_value("2022-11-28"));

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("unable to create reqwest client")
}

/// **Create an header value from a str**
///
/// Example: `header_value("application/json")`
fn header_value(src: &str) -> HeaderValue {
    HeaderValue::from_str(src).expect("unable to create header value")
}
