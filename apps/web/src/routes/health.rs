use axum::{extract::State, Json};
use serde::Serialize;

use crate::config::AppConfig;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: String,
}

pub async fn handler(State(config): State<AppConfig>) -> Json<HealthResponse>{
    Json(HealthResponse {
        status: "ok",
        version: config.version,
    })
}
