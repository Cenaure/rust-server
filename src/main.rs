#![allow(unused)]

use std::net::SocketAddr;
use axum::response::Html;
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello world! ")})
    );

    // region: ---Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");
    println!("->> Listening on http://{}", addr);

    axum::serve(listener, routes_hello)
        .await
        .expect("Failed to run server");
    // endregion: ---Start Server
}
