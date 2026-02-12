use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod db;
mod error;
mod handlers;
mod models;
mod utils;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::shorten_url,
        handlers::redirect_url,
        handlers::get_stats,
        handlers::generate_qr
    ),
    components(
        schemas(models::CreateUrlRequest, models::UrlResponse, models::StatsResponse, models::VisitStats)
    ),
    tags(
        (name = "url-shortener", description = "URL Shortener API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize DB
    let pool = db::init_db().await?;

    // Static files
    let static_files = ServeDir::new("static");

    // Router
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/shorten", post(handlers::shorten_url))
        .route("/{code}", get(handlers::redirect_url))
        .route("/stats/{code}", get(handlers::get_stats))
        .route("/qr/{code}", get(handlers::generate_qr))
        .fallback_service(static_files)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        )
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
