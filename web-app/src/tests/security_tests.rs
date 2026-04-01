use crate::server::apply_security_headers;
use axum::http::header;
use axum::{response::Response, routing::get, Router};
use tower::ServiceExt;

#[tokio::test]
async fn test_security_headers_present() {
    let app = Router::new().route("/", get(|| async { "ok" }));
    let app = apply_security_headers(app, false);

    let response: Response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response
            .headers()
            .get(header::X_CONTENT_TYPE_OPTIONS)
            .unwrap()
            .to_str()
            .unwrap(),
        "nosniff"
    );
    assert_eq!(
        response
            .headers()
            .get(header::X_FRAME_OPTIONS)
            .unwrap()
            .to_str()
            .unwrap(),
        "DENY"
    );
}

#[tokio::test]
async fn test_hsts_absent_by_default() {
    let app = Router::new().route("/", get(|| async { "ok" }));
    let app = apply_security_headers(app, false);

    let response: Response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(response
        .headers()
        .get(header::STRICT_TRANSPORT_SECURITY)
        .is_none());
}

#[tokio::test]
async fn test_hsts_present_in_prod() {
    let app = Router::new().route("/", get(|| async { "ok" }));
    let app = apply_security_headers(app, true);

    let response: Response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(response
        .headers()
        .get(header::STRICT_TRANSPORT_SECURITY)
        .is_some());

    assert_eq!(
        response
            .headers()
            .get(header::STRICT_TRANSPORT_SECURITY)
            .unwrap()
            .to_str()
            .unwrap(),
        "max-age=31536000; includeSubDomains"
    );
}
