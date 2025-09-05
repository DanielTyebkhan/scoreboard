use serde::Serialize;
#[derive(Debug, Serialize)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: HealthStatus,
}

impl HealthResponse {
    pub fn healthy() -> Self {
        HealthResponse {
            status: HealthStatus::Healthy,
        }
    }
    pub fn unhealthy() -> Self {
        HealthResponse {
            status: HealthStatus::Unhealthy,
        }
    }
}
