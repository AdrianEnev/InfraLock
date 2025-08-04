pub mod metrics;

use axum::{
    body::Body,
    extract::{FromRef, State},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::handlers::{self, AppState};
use crate::middleware::api_key_auth::{self, ApiKeyAuthState};
use crate::clients::web_api::WebApiClient;

// This allows us to extract the WebApiClient from the AppState
// in our route handlers
#[derive(Clone)]
struct ApiState {
    web_api: Arc<WebApiClient>,
}

// Implement FromRef to extract ApiState from AppState
impl FromRef<AppState> for ApiState {
    fn from_ref(state: &AppState) -> Self {
        ApiState {
            web_api: state.web_api_client.clone(),
        }
    }
}

// Helper function to create the router with state
pub fn create_router(state: AppState, auth_state: Arc<ApiKeyAuthState>) -> Router {
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

    // Apply auth middleware to protected routes
    let protected_routes = protected_routes.layer(
        middleware::from_fn_with_state(
            auth_state,
            |state: State<Arc<ApiKeyAuthState>>, request: Request<Body>, next: Next| async move {
                api_key_auth::api_key_auth(state, request, next).await
            },
        )
    );

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