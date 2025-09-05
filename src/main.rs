mod responses;
use crate::responses::health::HealthResponse;
use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;

const IS_HEALTHY: bool = true;

async fn health_check() -> axum::Json<HealthResponse> {
    let res = match IS_HEALTHY {
        true => HealthResponse::healthy(),
        false => HealthResponse::unhealthy(),
    };
    axum::Json(res)
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/health", get(health_check));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let tcp = TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcp, router).await.unwrap();
}
