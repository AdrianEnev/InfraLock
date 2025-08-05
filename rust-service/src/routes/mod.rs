pub mod metrics;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::handlers::{self, AppState};

// Helper function to create the router with state
pub fn create_router(state: AppState) -> Router {
    // Create the shared state
    let shared_state = Arc::new(state);

    // Public routes that don't require authentication
    let public_routes = Router::new()
        .route("/health", get(|| async { "OK" }));

    // Protected routes that require authentication
    let protected_routes = Router::new()
        .route("/api/lookup/self", get(handlers::lookup_self))
        .route("/api/lookup/{ip}", get(handlers::lookup_ip))
        .route("/api/threat-score/{ip}", get(handlers::get_threat_score))
        .route("/api/threat-score/self", get(handlers::get_self_threat_score))
        .route("/api/tor/{ip_or_range}", get(handlers::is_tor_exit_node))
        .route("/api/vpn/{ip_or_range}", get(handlers::is_vpn_or_datacenter))
        .route("/api/proxy/{ip_or_range}", get(handlers::is_proxy));

    // Debug routes
    let debug_routes = Router::new()
        .route("/debug/reset-circuit-breaker", post(reset_circuit_breaker));

    // Combine all routes with the shared state
    public_routes
        .merge(protected_routes)
        .merge(debug_routes)
        .with_state(shared_state)
        .layer(TraceLayer::new_for_http())
}

async fn reset_circuit_breaker(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    state.web_api_client.reset_circuit_breaker().await;
    (StatusCode::OK, "Circuit breaker reset")
}