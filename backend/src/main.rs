mod config;
mod db;
mod errors;
mod middleware;
mod models;
mod routes;
mod services;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "digitaria_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load config
    let config = config::AppConfig::from_env();
    let addr = format!("{}:{}", config.host, config.port);

    // Initialize database
    let db = db::connect(&config.database_url).await;
    tracing::info!("Database connected successfully");

    // Build application state
    let state = AppState {
        db,
        config: config.clone(),
    };

    // Build router
    let app = Router::new()
        .nest("/api/v1", routes::api_routes())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("🚀 Digitaria API running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

/// Shared application state available to all handlers
#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub config: config::AppConfig,
}
