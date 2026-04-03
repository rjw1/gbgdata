#![recursion_limit = "2048"]
#[cfg(feature = "ssr")]
use axum::http::header;
#[cfg(feature = "ssr")]
use axum::response::IntoResponse;

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
    use axum::routing::get;
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::postgres::PgPoolOptions;
    use tower_http::services::ServeDir;
    use tower_sessions::{Expiry, SessionManagerLayer};
    use tower_sessions_sqlx_store::PostgresStore;
    use web_app::app::*;
    use web_app::export::*;

    dotenvy::dotenv().ok();

    use tracing_subscriber::{fmt, prelude::*, EnvFilter};
    let _ = tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,web_app=info")),
        )
        .try_init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Run database migrations
    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    // Session setup
    let session_store = PostgresStore::new(pool.clone());
    session_store
        .migrate()
        .await
        .expect("Failed to migrate session store");

    let leptos_env = std::env::var("LEPTOS_ENV")
        .map(|v| v.to_lowercase())
        .unwrap_or_default();
    let is_prod = leptos_env == "prod" || leptos_env == "production";

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(is_prod)
        .with_expiry(Expiry::OnInactivity(
            tower_sessions::cookie::time::Duration::days(7),
        ));

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
        .nest_service(
            "/pkg",
            ServeDir::new(format!("{}/pkg", &*leptos_options.site_root)),
        )
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
        ));

    let app = web_app::server::apply_security_headers(app, is_prod);

    let app = app
        .layer(axum::middleware::from_fn(
            web_app::server::site_auth_middleware,
        ))
        .layer(session_layer)
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
