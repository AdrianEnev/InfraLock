use axum::{
    extract::State,
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::handlers::{self, AppState};

// Helper function to create the router with state
pub fn create_router(state: AppState) -> Router {
    let shared_state = Arc::new(state);

    // Create and return the router with the shared state
    Router::new()
        // Returns self IP:
        // Location (city?, country?, coordinates?)
        // VPN/Datacenter (true/false)
        .route(
            "/api/lookup/self",
            get({
                let state = Arc::clone(&shared_state);
                move |connect_info| handlers::lookup_self(State(state), connect_info)
            }),
        )

        // Returns specified IP:
        // Location (city?, country?, coordinates?)
        // VPN/Datacenter (true/false)
        .route(
            "/api/lookup/{ip}",
            get({
                let state = Arc::clone(&shared_state);
                move |path| handlers::lookup_ip(path, State(state))
            }),
        )

        // Check if ip is vpn or datacenter
        // Takes in a network range (CIDR) (also works if a regular ip is passed)
        .route(
            "/api/is_vpn_or_datacenter/{*ip}",
            get({
                move |path| handlers::is_vpn_or_datacenter(path)
            }),
        )

        // Add to the create_router function
        .route(
            "/api/is_proxy/{*ip}",
            get({
                move |path| handlers::is_proxy(path)
            }),
        )

        // Health check endpoint
        .route("/api/health", get(handlers::health_check))
        
        // Add tracing
        .layer(TraceLayer::new_for_http())
}