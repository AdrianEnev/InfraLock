// src/validation.rs
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use axum::http::HeaderMap;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum IpValidationError {
    #[error("Invalid IP address: {0}")]
    InvalidIpAddress(String),
    
    #[error("Missing required headers: 'X-Forwarded-For' or 'X-Real-IP'")]
    MissingIpHeaders,
    
    #[error("IP address not allowed: {0}")]
    NotAllowed(String),
}

/// Validates if the IP address is allowed for lookups
pub fn validate_ip(ip: IpAddr) -> Result<(), IpValidationError> {
    match ip {
        // Unspecified (0.0.0.0, ::)
        IpAddr::V4(ip) if ip.is_unspecified() => 
            Err(IpValidationError::NotAllowed("unspecified address (0.0.0.0)".into())),
        IpAddr::V6(ip) if ip.is_unspecified() => 
            Err(IpValidationError::NotAllowed("unspecified address (::)".into())),
        
        // Loopback (127.0.0.0/8, ::1)
        IpAddr::V4(ip) if ip.is_loopback() => 
            Err(IpValidationError::NotAllowed("loopback address (127.0.0.0/8)".into())),
        IpAddr::V6(ip) if ip.is_loopback() => 
            Err(IpValidationError::NotAllowed("loopback address (::1)".into())),
        
        // Private addresses
        IpAddr::V4(ip) if is_private_ipv4(&ip) => 
            Err(IpValidationError::NotAllowed("private address range".into())),
        
        // Link-local addresses
        IpAddr::V4(ip) if ip.is_link_local() => 
            Err(IpValidationError::NotAllowed("IPv4 link-local address (169.254.0.0/16)".into())),
        IpAddr::V6(ip) if ip.is_unicast_link_local() => 
            Err(IpValidationError::NotAllowed("IPv6 link-local address (fe80::/10)".into())),
        
        // Documentation addresses
        IpAddr::V4(ip) if is_documentation_ipv4(&ip) => 
            Err(IpValidationError::NotAllowed("documentation address (TEST-NET-1/2/3)".into())),
        IpAddr::V6(ip) if is_documentation_ipv6(&ip) => 
            Err(IpValidationError::NotAllowed("documentation address (2001:db8::/32)".into())),
        
        // Valid IP
        _ => Ok(()),
    }
}

/// Checks if an IPv4 address is in a private range
fn is_private_ipv4(ip: &Ipv4Addr) -> bool {
    match ip.octets() {
        [10, _, _, _] => true,                                // 10.0.0.0/8
        [172, b, _, _] if (16..=31).contains(&b) => true,     // 172.16.0.0/12
        [192, 168, _, _] => true,                            // 192.168.0.0/16
        _ => false,
    }
}

/// Checks if an IPv4 address is in a documentation range
fn is_documentation_ipv4(ip: &Ipv4Addr) -> bool {
    match ip.octets() {
        [192, 0, 2, _] => true,                              // 192.0.2.0/24 (TEST-NET-1)
        [198, 51, 100, _] => true,                           // 198.51.100.0/24 (TEST-NET-2)
        [203, 0, 113, _] => true,                            // 203.0.113.0/24 (TEST-NET-3)
        _ => false,
    }
}

/// Checks if an IPv6 address is in a documentation range
fn is_documentation_ipv6(ip: &Ipv6Addr) -> bool {
    // 2001:db8::/32 - Documentation prefix
    ip.segments()[0] == 0x2001 && (ip.segments()[1] & 0xffff) == 0xdb8
}

/// Extracts the client IP address from request headers
/// Returns an error if no valid IP could be extracted from headers
pub fn extract_client_ip(headers: &HeaderMap) -> Result<IpAddr, IpValidationError> {
    // Try X-Forwarded-For first (comma-separated list of IPs)
    if let Some(forwarded_for) = headers.get("x-forwarded-for") {
        let forwarded_for_str = forwarded_for.to_str().map_err(|_| 
            IpValidationError::InvalidIpAddress("Invalid X-Forwarded-For header".to_string())
        )?;
        
        if let Some(first_ip) = forwarded_for_str.split(',').next() {
            let trimmed_ip = first_ip.trim();
            return trimmed_ip.parse().map_err(|_| 
                IpValidationError::InvalidIpAddress(
                    format!("Invalid IP in X-Forwarded-For header: {}", trimmed_ip)
                )
            );
        }
    }

    // Try X-Real-IP
    if let Some(real_ip) = headers.get("x-real-ip") {
        let ip_str = real_ip.to_str().map_err(|_| 
            IpValidationError::InvalidIpAddress("Invalid X-Real-IP header".to_string())
        )?;
        
        return ip_str.parse().map_err(|_| 
            IpValidationError::InvalidIpAddress(
                format!("Invalid IP in X-Real-IP header: {}", ip_str)
            )
        );
    }

    // No valid IP found in headers
    Err(IpValidationError::MissingIpHeaders)
}