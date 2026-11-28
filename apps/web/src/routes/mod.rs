// register the sibling files
mod health;
mod home;
mod info;

use axum::{routing::get,Router};
use crate::config::AppConfig;

// Wire URLs to hanlers and atttach shared state
pub fn build_router(config: AppConfig) -> Router {
    Router::new()
        .route("/", get(home::handler))
        .route("/health", get(health::handler))
        .route("/api/info", get(info::handler))
        .with_state(config)
}
