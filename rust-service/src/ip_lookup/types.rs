use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

/// Categories for IP addresses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IpCategory {
    /// IP belongs to a VPN or datacenter
    Vpn,
    /// IP is an HTTP proxy
    ProxyHttp,
    /// IP is a SOCKS4 proxy
    ProxySocks4,
    /// IP is a SOCKS5 proxy
    ProxySocks5,
    /// IP is a TOR exit node
    TorExitNode,
}

impl std::fmt::Display for IpCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vpn => write!(f, "vpn"),
            Self::ProxyHttp => write!(f, "http_proxy"),
            Self::ProxySocks4 => write!(f, "socks4_proxy"),
            Self::ProxySocks5 => write!(f, "socks5_proxy"),
            Self::TorExitNode => write!(f, "tor_exit_node"),
        }
    }
}

impl FromStr for IpCategory {
    type Err = IpRangeError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "vpn" => Ok(Self::Vpn),
            "http" | "http_proxy" => Ok(Self::ProxyHttp),
            "socks4" | "socks4_proxy" => Ok(Self::ProxySocks4),
            "socks5" | "socks5_proxy" => Ok(Self::ProxySocks5),
            "tor" | "tor_exit" | "tor_exit_node" => Ok(Self::TorExitNode),
            _ => Err(IpRangeError::UnknownCategory(format!("Unknown IP category: {}", s))),
        }
    }
}

/// Represents a range of IP addresses with associated metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpRange {
    /// The network in CIDR notation (e.g., "192.168.1.0/24")
    pub network: String,
    /// The category of this IP range
    pub category: IpCategory,
    /// Source of this IP range (e.g., "vpn-provider-1")
    pub source: String,
    /// The format of the source data (e.g., Default, IpPort, TorExitList)
    pub format: SourceFormat,
    /// When this range was first seen
    pub first_seen: DateTime<Utc>,
    /// When this range was last updated
    pub last_updated: DateTime<Utc>,
}

impl IpRange {
    /// Create a new IP range with the current timestamp
    pub fn new(network: impl Into<String>, category: IpCategory, source: impl Into<String>, format: SourceFormat) -> Self {
        let now = Utc::now();
        Self {
            network: network.into(),
            category,
            source: source.into(),
            format,
            first_seen: now,
            last_updated: now,
        }
    }

    /// Update the last_updated timestamp to now
    pub fn touch(&mut self) {
        self.last_updated = Utc::now();
    }
}

/// Errors that can occur during IP range operations
#[derive(Debug, Error)]
pub enum IpRangeError {
    #[error("Invalid IP network: {0}")]
    InvalidNetwork(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Unknown category: {0}")]
    UnknownCategory(String),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

impl From<std::net::AddrParseError> for IpRangeError {
    fn from(err: std::net::AddrParseError) -> Self {
        Self::InvalidNetwork(err.to_string())
    }
}

/// Result type for IP range operations
pub type Result<T> = std::result::Result<T, IpRangeError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceFormat {
    /// Plain IP or CIDR (default)
    Default,
    /// IP:PORT format (extracts just the IP part)
    IpPort,
    /// Tor exit node list format
    TorExitList,
    // Add more formats as needed
}

impl Default for SourceFormat {
    fn default() -> Self {
        Self::Default
    }
}