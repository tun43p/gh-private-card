mod core;
mod features;

use dotenv::dotenv;
use std::{fs::create_dir_all, net::SocketAddr, path::Path};

use crate::core::routes::create_routes;

pub async fn run() {
    dotenv().ok();

    if !Path::new("static/").exists() {
        create_dir_all("static").expect("error creating directory");
    }

    let router = create_routes();
    let socket_addr = SocketAddr::from((
        get_server_host(),
        std::env::var("SERVER_PORT")
            .expect("missing SERVER_PORT in your .env file.")
            .parse()
            .unwrap(),
    ));

    println!("Listening on {}", socket_addr);

    axum::Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

/// **Get the application server host from your .env file.**
fn get_server_host() -> [u8; 4] {
    let server_host_vec = Vec::from_iter(
        std::env::var("SERVER_HOST")
            .expect("missing SERVER_HOST in your .env file.")
            .split('.')
            .map(String::from),
    );

    // Convert our Vec<String> into an [u8;4] array
    let mut server_host = [0; 4];
    for (i, s) in server_host_vec.iter().enumerate() {
        server_host[i] = s.parse::<u8>().unwrap();
    }

    server_host
}
