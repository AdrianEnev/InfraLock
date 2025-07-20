// tests/api_tests.rs
use axum_test::TestServer;
use your_crate::{create_router, AppState};
use tokio::sync::RwLock;
use std::sync::Arc;
use maxminddb::Reader;
use vpn_detection::VpnDetector;
use std::net::IpAddr;

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

#[tokio::test]
async fn test_vpn_detection_endpoint() {
    let state = Arc::new(AppState { /* your test state */ });
    let app = create_router(state);
    let server = TestServer::new(app).unwrap();
    
    // Test with known VPN IP (adjust based on your test data)
    let response = server.get("/api/vpn_datacentre/1.1.1.1").await;
    assert_eq!(response.status_code(), 200);
    assert_eq!(response.json::<bool>().await, true);
    
    // Test with regular IP
    let response = server.get("/api/vpn_datacentre/192.168.1.1").await;
    assert_eq!(response.status_code(), 200);
    assert_eq!(response.json::<bool>().await, false);
}