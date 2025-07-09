use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod error;
mod handlers;
mod routes;
mod types;

use crate::{
    config::Settings,
    handlers::AppState,
    routes::create_router,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let settings = Settings::new()?;
    
    // Clone the db_path to avoid moving settings
    let db_path = settings.resolve_db_path().unwrap_or_else(|e| {
        panic!("Failed to resolve database path: {}", e);
    });
    let reader = maxminddb::Reader::open_readfile(db_path)?;
    
    // Create application state
    let state = AppState { 
        maxmind_reader: Arc::new(RwLock::new(reader)) 
    };

    // Create router
    let app = create_router(state);

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
