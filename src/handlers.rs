use axum::{
    extract::{Path, State, ConnectInfo},
    Json,
};
use maxminddb::geoip2;
use serde::Serialize;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{error::AppError};
use crate::types::location_types::GeoInfo;

#[derive(Debug, Clone)]
pub struct AppState {
    pub maxmind_reader: Arc<RwLock<maxminddb::Reader<Vec<u8>>>>,
}

#[derive(Debug, Serialize)]
pub struct LookupResponse {
    pub ip: String,
    pub geo_info: GeoInfo,
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

    Ok(Json(LookupResponse {
        ip: ip_addr.to_string(),
        geo_info,
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

    Ok(Json(LookupResponse {
        ip: addr.ip().to_string(),
        geo_info,
    }))
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