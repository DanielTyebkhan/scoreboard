use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn health_check() -> &'static str {
    "API running"
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/health", get(health_check));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let tcp = TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcp, router).await.unwrap();
}
