// Expose these modules so main.rs and test/ can import them
pub mod config;
pub mod healthcheck;
pub mod routes;

// Re-export key items at the crate root for shorter imports
pub use config::AppConfig;
pub use routes::build_router;
