use axum::{routing::get, Json, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn ping() -> Json<&'static str> {
    Json(aider_core::ping())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ping", get(ping));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {addr}");
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
