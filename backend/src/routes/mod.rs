pub mod auth;
pub mod invitations;
pub mod templates;

use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

use crate::AppState;

/// Compose all API sub-routes.
pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check))
        .nest("/auth", auth::routes())
        .nest("/templates", templates::routes())
        .nest("/invitations", invitations::routes())
    // .nest("/rsvp", rsvp::routes())
    // .nest("/payments", payments::routes())
    // .nest("/media", media::routes())
}

/// Health check endpoint — useful for Docker healthcheck and monitoring.
async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "digitaria-api",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
