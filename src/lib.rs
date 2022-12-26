mod helpers;
mod models;
mod routes;

use dotenv::dotenv;
use helpers::env;
use routes::create_routes;
use std::net::SocketAddr;

pub async fn run() {
    dotenv().ok();

    let router = create_routes();
    let socket_addr = SocketAddr::from((env::get_server_host(), env::get_server_port()));

    println!("Listening on {}", socket_addr);

    axum::Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
