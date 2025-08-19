use axum::{
    body::Body, extract::{ConnectInfo, Path, State}, Json
};
use serde::Serialize;
use std::net::{IpAddr};
use std::sync::Arc;
use tokio::sync::RwLock;    

use axum::extract::Request;

use crate::{
    errors::{
        validation::{
            extract_client_ip, validate_ip
        }, AppError
    }, services::lookup_service::LookupService
};
use crate::services::vpn_detection::VpnDetector;
use crate::services::proxy_detection::ProxyDetector;
use crate::services::tor_detection::TorDetector;
use crate::models::location::{GeoInfo, AsnInfo};
use percent_encoding::{percent_decode_str};
use crate::models::threat_score::ThreatScore;
use crate::ip_lookup::IpLookupService;
use moka::sync::Cache;

// Removed WebApiClient dependency; rust-service no longer performs API key validation

#[derive(Debug, Clone)]
pub struct AppState {
    pub maxmind_reader: Arc<RwLock<maxminddb::Reader<Vec<u8>>>>,
    pub asn_reader: Arc<RwLock<maxminddb::Reader<Vec<u8>>>>, // ASN DB reader
    pub lookup_cache: Arc<Cache<IpAddr, LookupResponse>>,
    pub ip_lookup_service: Arc<IpLookupService>,
}

#[derive(Debug, Serialize, Clone)]
pub struct LookupResponse {
    pub ip: String,
    pub geo_info: Option<GeoInfo>,
    pub asn_info: Option<AsnInfo>,
    pub is_vpn_or_datacenter: bool,
    pub is_proxy: bool,
    pub proxy_type: Option<&'static str>,
    pub is_tor_exit_node: bool,
    pub threat_score: u8,  // 0-100 threat score
    pub threat_details: Vec<String>,  // Descriptions of threats found
    pub recommended_action: String,  // Recommended response action (allow/challenge/block/redirect/monitor)
}

#[derive(Debug, Serialize)]
pub struct ThreatScoreResponse {
    pub ip: String,
    pub threat_score: u8,
    pub threat_details: Vec<String>,
}

#[axum::debug_handler]
pub async fn lookup_ip(
    Path(ip): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<LookupResponse>, AppError> {
    let ip_addr: IpAddr = ip.parse()?;
    
    // IP validation
    if let Err(e) = validate_ip(ip_addr) {
        tracing::warn!("Rejected IP lookup for {}: {}", ip_addr, e);
        return Err(AppError::ValidationError(e));
    }

    let lookup_service = LookupService::new(
        Arc::clone(&state.maxmind_reader),
        Arc::clone(&state.asn_reader),
        state.lookup_cache.clone(),
        Arc::clone(&state.ip_lookup_service),
    );

    let response = lookup_service.lookup_ip(ip_addr).await?;
    Ok(Json(response))
}

#[axum::debug_handler]
pub async fn lookup_self(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
) -> Result<Json<LookupResponse>, AppError> {
    // First, check if we have any of the required headers
    let headers = request.headers();
    
    // Log available headers for debugging
    tracing::debug!("Available headers: {:?}", headers.keys().map(|h| h.as_str()).collect::<Vec<_>>());
    
    // Extract and validate IP from headers
    let ip_addr = extract_client_ip(headers)
        .map_err(|e| {
            tracing::warn!("IP extraction failed: {}", e);
            AppError::from(e)
        })?;

    // Log the IP for debugging
    tracing::debug!("Client IP: {}", ip_addr);
    
    // Validate the IP
    if let Err(e) = validate_ip(ip_addr) {
        tracing::warn!("Rejected IP lookup for {}: {}", ip_addr, e);
        return Err(AppError::from(e));
    }

    let lookup_service = LookupService::new(
        Arc::clone(&state.maxmind_reader),
        Arc::clone(&state.asn_reader),
        state.lookup_cache.clone(),
        Arc::clone(&state.ip_lookup_service),
    );

    let response = lookup_service.lookup_ip(ip_addr).await?;
    tracing::debug!("Response: {:#?}", response);

    Ok(Json(response))
}

#[axum::debug_handler]
pub async fn get_threat_score(
    Path(ip): Path<String>,
) -> Result<Json<ThreatScoreResponse>, AppError> {
    let ip_addr: IpAddr = ip.parse().map_err(|_| {
        AppError::from(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid IP address format",
        ))
    })?;

    // IP validation
    if let Err(e) = validate_ip(ip_addr) {
        tracing::warn!("Rejected IP lookup for {}: {}", ip_addr, e);
        return Err(AppError::ValidationError(e));
    }

    // Get the necessary detection results
    let vpn_detector = VpnDetector::get();
    let is_vpn = vpn_detector.is_vpn_or_datacenter(ip_addr);
    
    let proxy_detector = ProxyDetector::get();
    let proxy_type = proxy_detector.check_proxy(ip_addr);
    let is_proxy = proxy_type.is_some();
    
    let tor_detector = TorDetector::get();
    let is_tor = tor_detector.is_tor_exit_node(ip_addr);

    // Calculate threat score
    let threat_score = ThreatScore::from_ip_info(
        ip_addr,
        is_vpn,
        is_proxy,
        proxy_type,
        is_tor,
    );

    // Extract threat details
    let threat_details = threat_score.findings
        .iter()
        .map(|f| f.description.clone())
        .collect();

    Ok(Json(ThreatScoreResponse {
        ip: ip_addr.to_string(),
        threat_score: threat_score.score,
        threat_details,
    }))
}

#[axum::debug_handler]
pub async fn get_self_threat_score(
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
) -> Result<Json<ThreatScoreResponse>, AppError> {
    let ip_addr: IpAddr = addr.ip().to_string().parse().map_err(|_| {
        AppError::from(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid IP address format",
        ))
    })?;
    
    // IP validation
    if let Err(e) = validate_ip(ip_addr) {
        tracing::warn!("Rejected IP lookup for {}: {}", ip_addr, e);
        return Err(AppError::ValidationError(e));
    }

    // Get the necessary detection results
    let vpn_detector = VpnDetector::get();
    let is_vpn = vpn_detector.is_vpn_or_datacenter(ip_addr);
    
    let proxy_detector = ProxyDetector::get();
    let proxy_type = proxy_detector.check_proxy(ip_addr);
    let is_proxy = proxy_type.is_some();
    
    let tor_detector = TorDetector::get();
    let is_tor = tor_detector.is_tor_exit_node(ip_addr);

    // Calculate threat score
    let threat_score = ThreatScore::from_ip_info(
        ip_addr,
        is_vpn,
        is_proxy,
        proxy_type,
        is_tor,
    );

    // Extract threat details
    let threat_details = threat_score.findings
        .iter()
        .map(|f| f.description.clone())
        .collect();

    Ok(Json(ThreatScoreResponse {
        ip: ip_addr.to_string(),
        threat_score: threat_score.score,
        threat_details,
    }))
}

#[axum::debug_handler]
pub async fn is_tor_exit_node(
    Path(ip_or_range): Path<String>,
) -> Result<Json<TorResponse>, AppError> {
    // URL decode the path parameter to handle %2F in the URL
    let decoded = percent_decode_str(&ip_or_range)
        .decode_utf8()
        .map_err(|_| AppError::from(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Failed to decode URL-encoded input"
        )))?;
    
    let detector = TorDetector::get();
    
    // Try to parse as a single IP
    if let Ok(ip_addr) = decoded.parse::<IpAddr>() {
        let is_tor = detector.is_tor_exit_node(ip_addr);
        return Ok(Json(TorResponse { is_tor_exit_node: is_tor }));
    }
    
    // If we get here, it's not a valid IP
    Err(AppError::from(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        format!("Invalid IP address format: '{}'", decoded),
    )))
}

#[derive(Debug, Serialize)]
pub struct TorResponse {
    pub is_tor_exit_node: bool,
}

#[axum::debug_handler]
pub async fn is_vpn_or_datacenter(
    Path(ip_or_range): Path<String>,
) -> Result<String, AppError> {
    // URL decode the path parameter to handle %2F in the URL
    let decoded = percent_decode_str(&ip_or_range)
        .decode_utf8()
        .map_err(|_| AppError::from(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Failed to decode URL-encoded input"
        )))?;
    
    let detector = VpnDetector::get();
    
    // First try to parse as a single IP (Ex. 192.168.1.100)
    if let Ok(ip_addr) = decoded.parse::<IpAddr>() {
        let is_vpn = detector.is_vpn_or_datacenter(ip_addr);
        return Ok(format!("is_vpn/datacenter: {}", is_vpn));
    }
    
    // If that fails, try to parse as a network range (Ex. 192.168.1.0/24)
    if let Some(is_vpn) = detector.is_range_vpn_or_datacenter(&decoded) {
        return Ok(format!("contains_vpn/datacenter: {}", is_vpn));
    }
    
    // If we get here, it's not a valid IP or network range
    Err(AppError::from(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        format!("Invalid IP address or network range format: '{}'. Expected format: '1.2.3.4' or '1.2.3.0/24'", decoded),
    )))
}

#[derive(Debug, Serialize)]
pub struct ProxyResponse {
    pub is_proxy: bool,
    pub proxy_type: Option<&'static str>,
}

#[axum::debug_handler]
pub async fn is_proxy(
    Path(ip_or_range): Path<String>,
) -> Result<Json<ProxyResponse>, AppError> {
    // URL decode the path parameter
    let decoded = percent_decode_str(&ip_or_range)
        .decode_utf8()
        .map_err(|_| AppError::from(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Failed to decode URL-encoded input"
        )))?;
    
    let detector = ProxyDetector::get();
    
    // First try to parse as a single IP
    if let Ok(ip_addr) = decoded.parse::<IpAddr>() {
        let proxy_type = detector.check_proxy(ip_addr);
        return Ok(Json(ProxyResponse {
            is_proxy: proxy_type.is_some(),
            proxy_type,
        }));
    }
    
    // If that fails, try to parse as a network range
    if let Some(contains_proxy) = detector.is_range_proxy(&decoded) {
        return Ok(Json(ProxyResponse {
            is_proxy: contains_proxy,
            proxy_type: None, // We don't have type information for ranges
        }));
    }
    
    // If we get here, it's not a valid IP or network range
    Err(AppError::from(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        format!("Invalid IP address or network range format: '{}'. Expected format: '1.2.3.4' or '1.2.3.0/24'", decoded),
    )))
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: &'static str,
}

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION"),
    })
}

// Metrics handler
pub async fn metrics() -> Result<impl axum::response::IntoResponse, axum::http::StatusCode> {
    // For now, return empty metrics
    let metrics = Vec::new();
    
    match String::from_utf8(metrics) {
        Ok(metrics_string) => Ok((
            axum::response::AppendHeaders([
                (axum::http::header::CONTENT_TYPE, "text/plain; version=0.0.4"),
            ]),
            metrics_string,
        )),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;

    // Test health_check handler
    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert_eq!(response.0.status, "ok");
    }

    // Test lookup_ip with valid IP
    #[tokio::test]
    async fn test_lookup_ip_valid() {
        let state = setup_test_state();
        let ip = "8.8.8.8".to_string();
        let result = lookup_ip(Path(ip), State(state)).await;
        assert!(result.is_ok());
    }

    // Test lookup_ip with invalid IP
    #[tokio::test]
    async fn test_lookup_ip_invalid() {
        let state = setup_test_state();
        let ip = "invalid.ip".to_string();
        let result = lookup_ip(Path(ip), State(state)).await;
        assert!(result.is_err());
    }

    // Test lookup_self
    #[tokio::test]
    async fn test_lookup_self() {
        let state = setup_test_state();
        
        // Create a test request with X-Forwarded-For header
        let request = Request::builder()
            .uri("/lookup/self")
            .header("x-forwarded-for", "203.0.113.1, 198.51.100.1")
            .body(Body::empty())
            .unwrap();
            
        // Create a mock ConnectInfo
        let remote_addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
        let connect_info = ConnectInfo(remote_addr);
        
        // Insert ConnectInfo into extensions
        let (mut parts, body) = request.into_parts();
        parts.extensions.insert(connect_info);
        let request = Request::from_parts(parts, body);
        
        let result = lookup_self(State(state), request).await;
        assert!(result.is_ok());
        
        // The IP should be the first one from X-Forwarded-For
        let response = result.unwrap();
        assert_eq!(response.0.ip, "203.0.113.1");
        
        // Test with X-Real-IP header
        let state = setup_test_state();
        let request = Request::builder()
            .uri("/lookup/self")
            .header("x-real-ip", "192.0.2.1")
            .body(Body::empty())
            .unwrap();
            
        // Insert ConnectInfo into extensions
        let (mut parts, body) = request.into_parts();
        parts.extensions.insert(ConnectInfo(remote_addr));
        let request = Request::from_parts(parts, body);
        
        let result = lookup_self(State(state), request).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.ip, "192.0.2.1");
        
        // Test with direct connection (no headers)
        let state = setup_test_state();
        let request = Request::builder()
            .uri("/lookup/self")
            .body(Body::empty())
            .unwrap();
            
        // Insert ConnectInfo into extensions
        let (mut parts, body) = request.into_parts();
        parts.extensions.insert(ConnectInfo(remote_addr));
        let request = Request::from_parts(parts, body);
        
        let result = lookup_self(State(state), request).await;
        assert!(result.is_err());
    }

    fn setup_test_state() -> Arc<AppState> {
        // Setup a test MaxMind reader with test data
        unimplemented!()
    }
}