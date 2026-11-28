use std::env;

// tells the rust compiler to auto generate two traits at compile time
// Clone is required by axum so it can hand a copy of AppConfig to every incoming request
#[derive(Clone, Debug)]
pub struct AppConfig {
    pub version: String,
    pub hostname: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        // returns an Ok value or an Err value
        // instead of crashing on missing env vars, unwrap_or_else runs the closure
        let version = env::var("APP_VERSION").unwrap_or_else(|_| "local".to_string());
        let hostname = env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string());
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8000);

        Self { version, hostname, port }
    }
}
