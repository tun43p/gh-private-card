mod routes;

use routes::create_routes;
use std::net::SocketAddr;

pub async fn run() {
    let router = create_routes();
    let socket_addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
