use axum::{
    extract::{Path, State, ConnectInfo},
    Json,
};
use maxminddb::geoip2;
use serde::Serialize;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::errors::AppError;
use crate::services::vpn_detection::VpnDetector;
use crate::services::proxy_detection::ProxyDetector;
use crate::models::location::{GeoInfo, AsnInfo};
use crate::services::tor_detection::TorDetector;
use crate::models::location::GeoInfo;
use percent_encoding::{percent_decode_str};

#[derive(Debug, Clone)]
pub struct AppState {
    pub maxmind_reader: Arc<RwLock<maxminddb::Reader<Vec<u8>>>>,
    pub asn_reader: Arc<RwLock<maxminddb::Reader<Vec<u8>>>>, // ASN DB reader
}

#[derive(Debug, Serialize)]
pub struct LookupResponse {
    pub ip: String,
    pub geo_info: GeoInfo,
    pub asn_info: Option<AsnInfo>,
    pub is_vpn_or_datacenter: bool,
    pub is_proxy: bool,
    pub proxy_type: Option<&'static str>,
    pub is_tor_exit_node: bool,
}

#[axum::debug_handler]
pub async fn lookup_ip(
    Path(ip): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<LookupResponse>, AppError> {
    let ip_addr: IpAddr = ip.parse().map_err(|_| {
        AppError::from(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid IP address format",
        ))
    })?;

    let reader = state.maxmind_reader.read().await;
    let city: Option<geoip2::City> = reader.lookup(ip_addr)?;
    let geo_info = match city {
        Some(city) => GeoInfo::from(city),
        None => return Err(AppError::NotFound("IP address not found in database".to_string())),
    };

    // ASN lookup
    let asn_reader = state.asn_reader.read().await;
    let asn: Option<geoip2::Asn> = asn_reader.lookup(ip_addr)?;
    let asn_info = asn.as_ref().map(AsnInfo::from);

    let vpn_detector = VpnDetector::get();
    let is_vpn = vpn_detector.is_vpn_or_datacenter(ip_addr);
    
    let proxy_detector = ProxyDetector::get();
    let proxy_type = proxy_detector.check_proxy(ip_addr);
    let is_proxy = proxy_type.is_some();
    
    let tor_detector = TorDetector::get();
    let is_tor = tor_detector.is_tor_exit_node(ip_addr);

    Ok(Json(LookupResponse {
        ip: ip_addr.to_string(),
        geo_info,
        asn_info,
        is_vpn_or_datacenter: is_vpn,
        is_proxy,
        proxy_type,
        is_tor_exit_node: is_tor,
    }))
}

#[axum::debug_handler]
pub async fn lookup_self(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
) -> Result<Json<LookupResponse>, AppError> {
    let reader = state.maxmind_reader.read().await;
    let city: Option<geoip2::City> = reader.lookup(addr.ip())?;
    let geo_info = match city {
        Some(city) => GeoInfo::from(city),
        None => return Err(AppError::NotFound("IP address not found in database".to_string())),
    };

    // ASN lookup
    let asn_reader = state.asn_reader.read().await;
    let asn: Option<geoip2::Asn> = asn_reader.lookup(addr.ip())?;
    let asn_info = asn.as_ref().map(AsnInfo::from);

    let vpn_detector = VpnDetector::get();
    let is_vpn = vpn_detector.is_vpn_or_datacenter(addr.ip());
    
    let proxy_detector = ProxyDetector::get();
    let proxy_type = proxy_detector.check_proxy(addr.ip());
    let is_proxy = proxy_type.is_some();
    
    let tor_detector = TorDetector::get();
    let is_tor = tor_detector.is_tor_exit_node(addr.ip());

    Ok(Json(LookupResponse {
        ip: addr.ip().to_string(),
        geo_info,
        asn_info,
        is_vpn_or_datacenter: is_vpn,
        is_proxy,
        proxy_type,
        is_tor_exit_node: is_tor,
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
    
    // First try to parse as a single IP
    if let Ok(ip_addr) = decoded.parse::<IpAddr>() {
        let is_vpn = detector.is_vpn_or_datacenter(ip_addr);
        return Ok(format!("is_vpn/datacenter: {}", is_vpn));
    }
    
    // If that fails, try to parse as a network range
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

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::ConnectInfo;
    use std::net::SocketAddr;
    use std::str::FromStr;

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
        let addr = SocketAddr::from_str("127.0.0.1:8080").unwrap();
        let result = lookup_self(State(state), ConnectInfo(addr)).await;
        assert!(result.is_ok());
    }

    fn setup_test_state() -> Arc<AppState> {
        // Setup a test MaxMind reader with test data
        unimplemented!()
    }
}