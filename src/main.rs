use std::net::SocketAddr;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(healthcheck));
    let socket_addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn healthcheck() -> String {
    "OK".to_owned()
}
