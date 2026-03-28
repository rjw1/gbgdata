#[cfg(feature = "ssr")]
use axum::response::IntoResponse;
#[cfg(feature = "ssr")]
use axum::http::header;

#[cfg(feature = "ssr")]
async fn robots_txt() -> impl IntoResponse {
    match tokio::fs::read_to_string("site/robots.txt").await {
        Ok(s) => s,
        Err(_) => "User-agent: *\nDisallow: /".to_string(),
    }
}

#[cfg(feature = "ssr")]
async fn favicon_ico() -> impl IntoResponse {
    match tokio::fs::read("site/favicon.ico").await {
        Ok(b) => ([(header::CONTENT_TYPE, "image/x-icon")], b).into_response(),
        Err(_) => axum::http::StatusCode::NOT_FOUND.into_response(),
    }
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use axum::routing::get;
    use axum::http::{HeaderValue, header};
    use web_app::app::*;
    use web_app::export::*;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::postgres::PgPoolOptions;
    use tower_http::services::ServeDir;
    use tower_http::set_header::SetResponseHeaderLayer;

    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    
    let state = AppState {
        leptos_options: leptos_options.clone(),
        pool: pool.clone(),
    };

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .nest_service("/pkg", ServeDir::new(format!("{}/pkg", &*leptos_options.site_root)))
        .nest_service("/assets", ServeDir::new(&*leptos_options.site_root))
        .route("/robots.txt", get(robots_txt))
        .route("/favicon.ico", get(favicon_ico))
        .route("/export/json", get(export_json))
        .route("/export/csv", get(export_csv))
        .route("/export/parquet", get(export_parquet))
        .leptos_routes_with_context(
            &state,
            routes,
            {
                let pool = pool.clone();
                move || provide_context(pool.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::render_app_to_stream_with_context(
            {
                let pool = pool.clone();
                move || provide_context(pool.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("x-robots-tag"),
            HeaderValue::from_static("noindex, nofollow, noarchive, noai, noimageai"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("DENY"),
        ))
        .with_state(state);

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{}", &addr);
    axum::serve(listener, app).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // no-op for non-ssr feature
}
