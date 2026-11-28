use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use devops_rpi_web::{build_router, AppConfig};
use http_body_util::BodyExt;
use tower::ServiceExt;

fn test_config() -> AppConfig {
    AppConfig {
        version: "test-sha".to_string(),
        hostname: "test-host".to_string(),
        port: 8000,
    }
}

#[tokio::test]
async fn health_returns_ok_and_version() {
    let res = build_router(test_config())
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["status"], "ok");
    assert_eq!(body["version"], "test-sha");
}

#[tokio::test]
async fn info_returns_service_metadata() {
    let res = build_router(test_config())
        .oneshot(Request::builder().uri("/api/info").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["service"], "devops-rpi-web");
    assert_eq!(body["hostname"], "test-host");
}

#[tokio::test]
async fn home_renders_html_with_version() {
    let res = build_router(test_config())
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes.to_vec()).unwrap();
    assert!(html.contains("DevOps RPi"));
    assert!(html.contains("test-sha"));
}
