use std;

/// Get GitHub API token from your .env file
pub fn get_github_token() -> String {
    std::env::var("GITHUB_TOKEN").expect("missing GITHUB_TOKEN in your .env file.")
}

/// Get the application server host from your .env file
pub fn get_server_host() -> [u8; 4] {
    let server_host_vec = Vec::from_iter(
        std::env::var("SERVER_HOST")
            .expect("missing SERVER_HOST in your .env file.")
            .split(".")
            .map(String::from),
    );

    // Convert our Vec<String> into an [u8;4] array
    let mut server_host = [0; 4];
    for (i, s) in server_host_vec.iter().enumerate() {
        server_host[i] = s.parse::<u8>().unwrap();
    }

    server_host
}

/// Get the application server port from your .env file
pub fn get_server_port() -> u16 {
    std::env::var("SERVER_PORT")
        .expect("missing SERVER_PORT in your .env file.")
        .parse()
        .unwrap()
}
