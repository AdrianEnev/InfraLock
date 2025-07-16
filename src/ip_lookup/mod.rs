//! IP lookup module for efficient IP address and range lookups.
//! 
//! This module provides functionality for fast IP address lookups using a radix tree.
//! It's designed to work with various IP categories (VPN, proxy, TOR, etc.)
//! and integrates with the background updater for automatic updates.

pub mod tree;
pub mod types;
pub mod loader;
pub mod service;

// Re-export the main types for easier access
pub use tree::SharedRadixTree;
pub use types::IpCategory;
pub use service::{IpLookupService, IpLookupServiceConfig, IpRangeSource};

use std::net::IpAddr;
use std::sync::Arc;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::ip_lookup::types::SourceFormat;

/// Default path for storing IP range data
pub const DEFAULT_DATA_DIR: &str = "data/ip_ranges";

/// Global instance of the IP lookup service
static IP_LOOKUP_SERVICE: Lazy<RwLock<Option<Arc<IpLookupService>>>> = Lazy::new(|| RwLock::new(None));

/// Initialize the global IP lookup service with default configuration
pub async fn init_with_defaults() -> anyhow::Result<()> {
    let config = default_config()?;
    init_with_config(config).await
}

/// Initialize the global IP lookup service with a custom configuration
pub async fn init_with_config(config: IpLookupServiceConfig) -> anyhow::Result<()> {
    let mut service_guard = IP_LOOKUP_SERVICE.write().await;
    
    if service_guard.is_some() {
        return Err(anyhow::anyhow!("IP lookup service is already initialized"));
    }
    
    let service = Arc::new(IpLookupService::new(config));
    service.start_background_updates();
    *service_guard = Some(service);
    
    Ok(())
}

/// Get the global IP lookup service
pub async fn get_service() -> anyhow::Result<Arc<IpLookupService>> {
    loop {
        // First try to get a read lock
        let guard = IP_LOOKUP_SERVICE.read().await;
        if let Some(service) = guard.as_ref() {
            return Ok(service.clone());
        }
        drop(guard); // Release the read lock before trying to initialize
        
        // If we get here, the service needs to be initialized
        if let Err(e) = init_with_defaults().await {
            // If another task is initializing, we'll retry
            if !e.to_string().contains("already initialized") {
                return Err(e);
            }
            // Small delay to prevent tight loop
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }
}

/// Check if an IP address matches any known category
pub async fn check_ip(ip: IpAddr) -> anyhow::Result<Option<IpCategory>> {
    let service = get_service().await?;
    Ok(service.tree().lookup(ip))
}

/// Create a default configuration for the IP lookup service
pub fn default_config() -> anyhow::Result<IpLookupServiceConfig> {
    let data_dir = std::env::current_dir()?.join(DEFAULT_DATA_DIR);
    
    Ok(IpLookupServiceConfig {
        data_dir,
        check_updates: true,
        update_interval_secs: 3600, // 1 hour
        max_cache_age_secs: 86400,  // 24 hours
        sources: vec![
            // VPN list (working)
            IpRangeSource {
                url: "https://raw.githubusercontent.com/X4BNet/lists_vpn/main/output/datacenter/ipv4.txt".to_string(),
                category: IpCategory::Vpn,
                name: "x4net-vpn".to_string(),
                enabled: true,
                format: SourceFormat::Default,
            },
            // HTTP proxies (update format)
            IpRangeSource {
                url: "https://raw.githubusercontent.com/TheSpeedX/SOCKS-List/master/http.txt".to_string(),
                category: IpCategory::ProxyHttp,
                name: "thespeedx-http".to_string(),
                enabled: true,
                format: SourceFormat::IpPort,
            },
            // SOCKS4 proxies (update format)
            IpRangeSource {
                url: "https://raw.githubusercontent.com/TheSpeedX/SOCKS-List/master/socks4.txt".to_string(),
                category: IpCategory::ProxySocks4,
                name: "thespeedx-socks4".to_string(),
                enabled: true,
                format: SourceFormat::IpPort,
            },
            // SOCKS5 proxies (update format)
            IpRangeSource {
                url: "https://raw.githubusercontent.com/TheSpeedX/SOCKS-List/master/socks5.txt".to_string(),
                category: IpCategory::ProxySocks5,
                name: "thespeedx-socks5".to_string(),
                enabled: true,
                format: SourceFormat::IpPort,
            },
            // Tor exit nodes (special format)
            IpRangeSource {
                url: "https://check.torproject.org/exit-addresses".to_string(),
                category: IpCategory::TorExitNode,
                name: "tor-exit-nodes".to_string(),
                enabled: true,
                format: SourceFormat::TorExitList,
            },
        ],
    })
}