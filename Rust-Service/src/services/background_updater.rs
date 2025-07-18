//! Background updater for VPN and proxy data files.
//!
//! Periodically checks remote sources for updated files, compares with local versions, and updates if necessary.

use std::time::Duration;
use tokio::time::sleep;
use crate::utils::file_ops::{files_differ, atomic_replace};
use crate::utils::http_client::download_file;
use tempfile::TempDir;

/// Configuration for the background updater.
pub struct BackgroundUpdaterConfig {
    /// Remote URL for VPN list
    pub vpn_url: String,
    /// Remote URL for HTTP proxies
    pub http_proxy_url: String,
    /// Remote URL for SOCKS4 proxies
    pub socks4_proxy_url: String,
    /// Remote URL for SOCKS5 proxies
    pub socks5_proxy_url: String,
    /// Remote URL for Tor exit nodes
    pub tor_exit_nodes_url: String,
    /// How often to check for updates (seconds)
    pub interval_secs: u64,
    /// Local file paths
    pub vpn_path: String,
    pub http_proxy_path: String,
    pub socks4_proxy_path: String,
    pub socks5_proxy_path: String,
    pub tor_exit_nodes_path: String,
}

/// Main background updater struct.
pub struct BackgroundUpdater {
    pub config: BackgroundUpdaterConfig,
}

impl BackgroundUpdater {
    /// Create a new BackgroundUpdater with the given configuration.
    pub fn new(config: BackgroundUpdaterConfig) -> Self {
        Self { config }
    }

    /// Start the background update loop as a Tokio task.
    pub async fn start(self) {
        loop {
            if let Err(e) = self.check_and_update().await {
                eprintln!("[BackgroundUpdater] Error: {}", e);
            }
            sleep(Duration::from_secs(self.config.interval_secs)).await;
        }
    }

    /// Check and update all files if needed.
    async fn check_and_update(&self) -> std::io::Result<()> {
        std::fs::create_dir_all("data/tmp_update")?;
        let temp_dir = TempDir::new_in("data/tmp_update")?;
        self.check_one(
            &self.config.vpn_url,
            &self.config.vpn_path,
            &temp_dir
        ).await?;
        self.check_one(
            &self.config.http_proxy_url,
            &self.config.http_proxy_path,
            &temp_dir
        ).await?;
        self.check_one(
            &self.config.socks4_proxy_url,
            &self.config.socks4_proxy_path,
            &temp_dir
        ).await?;
        self.check_one(
            &self.config.socks5_proxy_url,
            &self.config.socks5_proxy_path,
            &temp_dir
        ).await?;
        // Tor Exit Nodes
        self.check_one(
            &self.config.tor_exit_nodes_url,
            &self.config.tor_exit_nodes_path,
            &temp_dir
        ).await?;
        // After all checks/updates
        // temp_dir is dropped here, and the directory + all files are deleted automatically
        Ok(())
    }

    /// Download, compare, and update a single file if needed.
    async fn check_one(&self, url: &str, local_path: &str, temp_dir: &TempDir) -> std::io::Result<()> {
        let temp_path = temp_dir.path().join("download.tmp");
        if let Err(e) = download_file(url, &temp_path).await {
            eprintln!("[BackgroundUpdater] Download error: {}", e);
            return Err(e);
        }
        let need_update = match files_differ(std::path::Path::new(local_path), &temp_path) {
            Ok(diff) => diff,
            Err(e) => {
                eprintln!("[BackgroundUpdater] Compare error: {}. Will update.", e);
                true
            }
        };
        if need_update {
            if let Err(e) = atomic_replace(&temp_path, std::path::Path::new(local_path)) {
                eprintln!("[BackgroundUpdater] Atomic replace error: {}", e);
                return Err(e);
            }
        }
        Ok(())
    }
}

// NOTE: Replace the temp file path strings with actual temp file locations, e.g., /tmp/vpn.txt, or use tempfile crate for safety.
// NOTE: Replace the URL strings in config with your actual remote file URLs.