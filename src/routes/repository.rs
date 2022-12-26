use crate::helpers::client;

/// Get repository informations from GitHub API
pub async fn get_repository() -> String {
    let client = client::create_client();

    // TODO(tun43p): Parse real url
    let url = "https://github.com/tun43p/vera";
    let url: String = url.replace("github.com/", "api.github.com/repos/");

    println!("{}", url);

    let result = client
        .get(url)
        .send()
        .await
        .expect("error getting response");

    let data = result.text().await.expect("error getting data");

    println!("{}", data);

    "Get repository card".to_owned()
}
