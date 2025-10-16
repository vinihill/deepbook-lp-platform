//! API server implementation

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub mod handlers;
pub mod routes;

/// Create the API router
pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/api/v1/vaults", get(handlers::list_vaults))
        .route("/api/v1/vaults/:id", get(handlers::get_vault))
        .route("/api/v1/deposit", post(handlers::deposit))
        .route("/api/v1/withdraw", post(handlers::withdraw))
        .route("/api/v1/strategies", get(handlers::list_strategies))
        .route("/api/v1/metrics", get(handlers::get_metrics))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

