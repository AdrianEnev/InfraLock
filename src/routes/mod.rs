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

         // Get threat score for a specific IP
         .route(
            "/api/threat_score/{ip}",
            get(handlers::get_threat_score),
        )

        // Get threat score for self IP
        .route(
            "/api/threat_score/self",
            get(handlers::get_self_threat_score),
        )

        // Check if IP is a VPN or datacenter
        // Takes in a network range (CIDR) (also works if a regular ip is passed)
        .route(
            "/api/is_vpn_or_datacenter/{*ip}",
            get(handlers::is_vpn_or_datacenter),
        )

        // Check if IP is a proxy
        .route(
            "/api/is_proxy/{*ip}",
            get(handlers::is_proxy),
        )
        
        // Check if IP is a Tor exit node
        .route(
            "/api/is_tor_exit_node/{ip}",
            get(handlers::is_tor_exit_node),
        )

        // Health check endpoint
        .route("/api/health", get(handlers::health_check))
        
        // Add tracing
        .layer(TraceLayer::new_for_http())
}