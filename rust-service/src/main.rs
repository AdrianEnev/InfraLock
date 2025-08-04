use std::sync::Arc;
use std::collections::HashSet;
use std::time::Duration;
use moka::sync::Cache;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use dotenv::dotenv;

use crate::clients::web_api::{WebApiClient, WebApiClientConfig};
use crate::middleware::api_key_auth::{ApiKeyAuthState};

mod alerting;
mod clients;
mod config;
mod errors;
mod handlers;
mod ip_lookup;
mod middleware;
mod models;
mod monitoring;
mod routes;
mod services;
mod utils;

use crate::config::Settings;
use crate::handlers::AppState;
use crate::routes::{create_router, metrics::metrics_routes};
use crate::services::background_updater::{BackgroundUpdater, BackgroundUpdaterConfig};

fn parse_unlimited_api_keys() -> HashSet<String> {
    std::env::var("UNLIMITED_API_KEYS")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv().ok();
    
    // Configure logging format based on environment
    tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env()
        .add_directive("debug".parse().unwrap())
        .add_directive("hyper=info".parse().unwrap())
        .add_directive("tower_http=info".parse().unwrap()))
    .with_target(true)
    .with_thread_ids(true)
    .init();

    tracing::info!("Starting geolocation service");
    tracing::debug!("Debug logging is enabled");

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
    
    // Parse unlimited API keys from environment
    let unlimited_api_keys = parse_unlimited_api_keys();
    tracing::info!("Loaded {} unlimited API keys", unlimited_api_keys.len());
    
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

    // Initialize Web API client for API key validation
    let web_api_config = WebApiClientConfig::default();
    let web_api_client = Arc::new(WebApiClient::new(web_api_config));

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
        lookup_cache,
        ip_lookup_service,
        web_api_client: web_api_client.clone(),  // Clone here for AppState
    };

    // Create auth state with a clone of web_api_client and unlimited API keys
    let auth_state = Arc::new(ApiKeyAuthState {
        web_api_client: web_api_client.clone(),  // Clone here for ApiKeyAuthState
        unlimited_api_keys,
    });
    
    // Create the main application router
    let app = create_router(state, auth_state);
    
    // Create the metrics router
    let metrics_router = metrics_routes();
    
    // Combine both routers
    let app = app.merge(metrics_router);

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
