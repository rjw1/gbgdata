use axum::http::{header, HeaderValue};
use axum::{response::Response, routing::get, Router};
use tower::ServiceExt;
use tower_http::set_header::SetResponseHeaderLayer;

#[tokio::test]
async fn test_security_headers_present() {
    let app = Router::new()
        .route("/", get(|| async { "ok" }))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ));

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
            .unwrap(),
        "nosniff"
    );
    assert_eq!(
        response.headers().get(header::X_FRAME_OPTIONS).unwrap(),
        "DENY"
    );
}

#[tokio::test]
async fn test_hsts_absent_by_default() {
    let app = Router::new().route("/", get(|| async { "ok" }));

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
