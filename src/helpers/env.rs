use std;

/// Get GitHub API token from your .env file
pub fn get_github_token() -> String {
    std::env::var("GITHUB_TOKEN").expect("missing GITHUB_TOKEN in your .env file.")
}

// TODO(tun43p): Parse server port
/// Get the application server host from your .env file
pub fn get_server_host() -> [u8; 4] {
    let server_host = std::env::var("SERVER_HOST").expect("missing SERVER_HOST in your .env file.");
    let server_host: Vec<String> = Vec::from_iter(server_host.split(".").map(String::from)); // [u8; 4] = [127, 0, 0, 1];

    println!("{}", server_host.first().unwrap());

    [127, 0, 0, 1]
}

/// Get the application server port from your .env file
pub fn get_server_port() -> u16 {
    std::env::var("SERVER_PORT")
        .expect("missing SERVER_PORT in your .env file.")
        .parse()
        .unwrap()
}
