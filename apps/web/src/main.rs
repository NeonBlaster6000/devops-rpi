use std::env;

use devops_rpi_web::{build_router, healthcheck, AppConfig};

async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };

    #[cfg(unix)]
    let terminate = async {
        if let Ok(mut sig) =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        {
            sig.recv().await;
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn serve(config: AppConfig) {
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind {addr}: {e}"));

    axum::serve(listener, build_router(config))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}

fn main() {
    let config = AppConfig::from_env();

    if env::args().any(|arg| arg == "--healthcheck") {
        std::process::exit(healthcheck::run(config.port));
    }

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime")
        .block_on(serve(config));
}
