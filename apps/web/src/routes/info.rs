use std::time::{SystemTime, UNIX_EPOCH};

use axum::{extract::State, Json};
use serde::Serialize;

use crate::config::AppConfig;

#[derive(Serialize)]
pub struct InfoResponse {
    pub service: &'static str,
    pub version: String,
    pub hostname: String,
    pub unix_time: u64,
}

pub async fn handler(State(config): State<AppConfig>) -> Json<InfoResponse> {
    let unix_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    Json(InfoResponse {
        service: "devops-rpi-web",
        version: config.version,
        hostname: config.hostname,
        unix_time,
    })
}
