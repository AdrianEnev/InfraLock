use moka::sync::Cache;
use std::time::Duration;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenv::dotenv;

mod config;
mod errors;
mod handlers;
mod routes;
mod models;
mod services;
mod utils;
mod ip_lookup;

use crate::config::Settings;
use crate::handlers::AppState;
use crate::routes::create_router;
use crate::services::background_updater::{BackgroundUpdater, BackgroundUpdaterConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv().ok();
    
    // Initialize tracing early to capture any startup logs
    
    // Initialize tracing
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::fmt::layer()
            .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
            .with_thread_ids(true)
            .with_target(false) // Disable target to reduce noise
            .with_level(true)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
    )
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "debug".into())
    )
    .init();

    // Load configuration
    let settings = Settings::new()?;

    // --- BackgroundUpdater configuration ---
    let updater_config = BackgroundUpdaterConfig {
        vpn_url: "https://raw.githubusercontent.com/X4BNet/lists_vpn/refs/heads/main/output/datacenter/ipv4.txt".to_string(),
        http_proxy_url: "https://raw.githubusercontent.com/TheSpeedX/SOCKS-List/master/http.txt".to_string(),
        socks4_proxy_url: "https://raw.githubusercontent.com/TheSpeedX/SOCKS-List/master/socks4.txt".to_string(),
        socks5_proxy_url: "https://raw.githubusercontent.com/TheSpeedX/SOCKS-List/master/socks5.txt".to_string(),
        tor_exit_nodes_url: "https://check.torproject.org/exit-addresses".to_string(),
        interval_secs: 86400, // 24 hours in seconds
        vpn_path: "data/vpns/ipv4.txt".to_string(),
        http_proxy_path: "data/proxies/http.txt".to_string(),
        socks4_proxy_path: "data/proxies/socks4.txt".to_string(),
        socks5_proxy_path: "data/proxies/socks5.txt".to_string(),
        tor_exit_nodes_path: "data/tor/exit-addresses.txt".to_string(),
    };
    
    let updater = BackgroundUpdater::new(updater_config);
    tokio::spawn(async move {
        updater.start().await;
    });
    // --- End BackgroundUpdater configuration ---
    
    // Clone the db_path to avoid moving settings
    let db_path = settings.resolve_db_path().unwrap_or_else(|e| {
        panic!("Failed to resolve database path: {}", e);
    });
    let reader = maxminddb::Reader::open_readfile(db_path)?;

    // ASN DB initialization
    let asn_db_path = settings.resolve_asn_db_path().unwrap_or_else(|e| {
        panic!("Failed to resolve ASN database path: {}", e);
    });
    let asn_reader = maxminddb::Reader::open_readfile(asn_db_path)?;

    // Initialize IP lookup service
    let ip_lookup_config = ip_lookup::default_config()?;
    let ip_lookup_service = Arc::new(ip_lookup::IpLookupService::new(ip_lookup_config));
    ip_lookup_service.start_background_updates();

    // In main.rs
    let ttl_seconds = std::env::var("CACHE_TTL_SECONDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3600); // default 1 hour

    let lookup_cache = Arc::new(
        Cache::builder()
            .time_to_live(Duration::from_secs(ttl_seconds))
            .max_capacity(100_000)
            .build()
        );
    
    // Create application state
    let state = AppState { 
        maxmind_reader: Arc::new(RwLock::new(reader)),
        asn_reader: Arc::new(RwLock::new(asn_reader)),
        lookup_cache: lookup_cache,
        ip_lookup_service: ip_lookup_service,
    };

    // Create router
    let app: axum::Router = create_router(state);

    // Run the server
    let addr = settings.server_addr();
    let listener = TcpListener::bind(addr).await?;
    
    tracing::info!("listening on {}", addr);
    
    // Use into_make_service_with_connect_info to enable ConnectInfo
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}
