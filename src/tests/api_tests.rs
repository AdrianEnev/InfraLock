// tests/api_tests.rs
use axum_test::TestServer;
use your_crate::{create_router, AppState};
use tokio::sync::RwLock;
use std::sync::Arc;
use maxminddb::Reader;

#[tokio::test]
async fn test_health_endpoint() {
    let state = Arc::new(AppState { /* test state */ });
    let app = create_router(state);
    let server = TestServer::new(app).unwrap();
    
    let response = server.get("/api/health").await;
    assert_eq!(response.status_code(), 200);
    assert_eq!(response.json::<serde_json::Value>()["status"], "ok");
}

#[tokio::test]
async fn test_ip_lookup_endpoint() {
    let state = Arc::new(AppState { /* test state */ });
    let app = create_router(state);
    let server = TestServer::new(app).unwrap();
    
    let response = server.get("/api/lookup/8.8.8.8").await;
    assert_eq!(response.status_code(), 200);
    // Add assertions for response body
}