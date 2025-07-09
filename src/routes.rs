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
        // Health check endpoint
        .route("/api/health", get(handlers::health_check))
        
        // IP lookup endpoint
        .route(
            "/api/lookup/{ip}",
            get({
                let state = Arc::clone(&shared_state);
                move |path| handlers::lookup_ip(path, State(state))
            }),
        )
        
        // Self lookup endpoint
        .route(
            "/api/lookup/self",
            get({
                let state = shared_state;
                move |connect_info| handlers::lookup_self(State(state), connect_info)
            }),
        )
        
        // Add tracing
        .layer(TraceLayer::new_for_http())
}