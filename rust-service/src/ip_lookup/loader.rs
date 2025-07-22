use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::net::IpAddr;
use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;
use filetime;
use tracing::{info, error};

use crate::ip_lookup::{
    service::IpRangeSource,
    types::{IpCategory, IpRange, IpRangeError, Result, SourceFormat, IpVersion},
};

/// Configuration for loading IP ranges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpRangeLoaderConfig {
    /// Base directory for storing downloaded files
    pub data_dir: PathBuf,
    /// Whether to check for updates on startup
    pub check_updates: bool,
    /// Maximum age of cached data before updating (in seconds)
    pub max_cache_age_secs: u64,
}

impl Default for IpRangeLoaderConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("data/ip_ranges"),
            check_updates: true,
            max_cache_age_secs: 86400, // 24 hours
        }
    }
}

/// Handles loading IP ranges from various sources
#[derive(Debug, Clone)]
pub struct IpRangeLoader {
    config: IpRangeLoaderConfig,
    http_client: Client,
}

impl IpRangeLoader {
    /// Create a new IP range loader with the given configuration
    pub fn new(config: IpRangeLoaderConfig) -> Self {
        Self {
            config,
            http_client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Load IP ranges from a file
    pub async fn load_from_file<P: AsRef<Path>>(
        &self,
        path: P,
        category: IpCategory,
        source: &str,
        format: SourceFormat,
    ) -> Result<Vec<IpRange>> {
        // For JsonList format, read the entire file and parse it as JSON
        if format == SourceFormat::JsonList {
            let content = tokio::fs::read_to_string(path.as_ref()).await.map_err(|e| {
                IpRangeError::IoError(io::Error::new(
                    e.kind(),
                    format!("Failed to read {}: {}", path.as_ref().display(), e),
                ))
            })?;
            
            // Create a temporary source to pass to parse_ranges
            let temp_source = IpRangeSource {
                url: "file://".to_string() + path.as_ref().to_str().unwrap_or(""),
                category,
                name: source.to_string(),
                enabled: true,
                format: SourceFormat::JsonList,
                ip_version: if path.as_ref().to_string_lossy().contains("_v6") {
                    IpVersion::V6
                } else {
                    IpVersion::V4
                },
            };
            
            return self.parse_ranges(&content, &temp_source);
        }
        
        // For other formats, use the line-by-line processing
        use tokio::io::AsyncBufReadExt;
        
        let file = tokio::fs::File::open(path.as_ref()).await.map_err(|e| {
            IpRangeError::IoError(io::Error::new(
                e.kind(),
                format!("Failed to open {}: {}", path.as_ref().display(), e),
            ))
        })?;

        let mut ranges = Vec::new();
        let now = Utc::now();
        let reader = tokio::io::BufReader::new(file);
        let mut lines = reader.lines();
        let mut line_num = 0;

        while let Some(line) = lines.next_line().await.map_err(|e| {
            IpRangeError::IoError(io::Error::new(
                e.kind(),
                format!("Error reading line {}: {}", line_num + 1, e),
            ))
        })? {
            line_num += 1;
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Handle different formats
            let network = match format {
                SourceFormat::TorExitList => {
                    if line.starts_with("ExitNode") || line.starts_with("Published") || line.starts_with("LastStatus") {
                        continue;
                    }
                    if let Some(ip_part) = line.split_whitespace().nth(1) {
                        if let Ok(ip) = ip_part.parse::<IpAddr>() {
                            match ip {
                                IpAddr::V4(_) => format!("{}/32", ip),
                                IpAddr::V6(_) => format!("{}/128", ip),
                            }
                        } else {
                            error!("Failed to parse IP address at line {}: '{}'", line_num, line);
                            continue;
                        }
                    } else {
                        continue;
                    }
                },
                SourceFormat::IpPort => {
                    if let Some(ip_str) = line.split(':').next() {
                        if let Ok(ip) = ip_str.parse::<IpAddr>() {
                            match ip {
                                IpAddr::V4(_) => format!("{}/32", ip_str),
                                IpAddr::V6(_) => format!("{}/128", ip_str),
                            }
                        } else {
                            error!("Failed to parse IP address at line {}: '{}'", line_num, line);
                            continue;
                        }
                    } else {
                        continue;
                    }
                },
                SourceFormat::Default => {
                    if line.contains('/') {
                        line.to_string()
                    } else if let Ok(ip) = line.parse::<IpAddr>() {
                        match ip {
                            IpAddr::V4(_) => format!("{}/32", line),
                            IpAddr::V6(_) => format!("{}/128", line),
                        }
                    } else {
                        error!("Failed to parse IP network at line {}: '{}'", line_num, line);
                        continue;
                    }
                },
                SourceFormat::JsonList => {
                    // This should never be reached due to the early return above
                    error!("Unexpected JsonList format in line processing loop");
                    continue;
                }
            };

            ranges.push(IpRange {
                network,
                category,
                source: source.to_string(),
                first_seen: now,
                last_updated: now,
                format,
            });
        }

        Ok(ranges)
    }

    /// Download IP ranges from a URL
    pub async fn download_ranges(
        &self,
        url: &str,
        source: &IpRangeSource,
    ) -> Result<Vec<IpRange>> {
        // Parse the URL
        let url_obj = Url::parse(url).map_err(|e| {
            IpRangeError::InvalidUrl(format!("Invalid URL '{}': {}", url, e))
        })?;

        // Create data directory if it doesn't exist
        tokio::fs::create_dir_all(&self.config.data_dir).await.map_err(|e| {
            IpRangeError::IoError(io::Error::new(
                e.kind(),
                format!("Failed to create data directory: {}", e),
            ))
        })?;

        // Generate a filename for this source
        let filename = self.filename_from_url(&url_obj, source.category, source.ip_version);
        let filepath = self.config.data_dir.join(&filename);
        
        // Download the file
        let content = self.download_file(url).await?;
        
        // Parse the content
        let ranges = self.parse_ranges(&content, source)?;
        
        // Save to file
        tokio::fs::write(&filepath, &content).await.map_err(|e| {
            IpRangeError::IoError(io::Error::new(
                e.kind(),
                format!("Failed to save file {}: {}", filepath.display(), e),
            ))
        })?;
        
        // Set the last modified time to now
        let now = Utc::now();
        let mtime = filetime::FileTime::from_system_time(now.into());
        if let Err(e) = filetime::set_file_mtime(&filepath, mtime) {
            error!("Failed to set last modified time for {}: {}", filepath.display(), e);
        }
        
        info!(
            "Downloaded and parsed {} ranges from {} (saved to {})",
            ranges.len(),
            url,
            filepath.display()
        );
        
        Ok(ranges)
    }

    /// Parse IP ranges from a string
    pub fn parse_ranges(
        &self,
        content: &str,
        source: &IpRangeSource,
    ) -> Result<Vec<IpRange>> {
        let mut ranges = Vec::new();
        //info!("Starting to parse ranges for source: {} (format: {:?})", source.name, source.format);
        
        // Handle JSON format first
        if source.format == SourceFormat::JsonList {
            //info!("Processing as JSON list");
            // Try to parse as JSON array (MISP warning list format)
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(content) {
                //info!("Parsing JSON content");
                
                // Handle MISP warning list format ({"list": ["cidr1", "cidr2", ...]})
                if let Some(list) = json_value.get("list").and_then(|l| l.as_array()) {
                    info!("Found MISP warning list format with {} entries", list.len());
                    for (i, cidr_value) in list.iter().enumerate() {
                        if let Some(cidr) = cidr_value.as_str() {
                            //info!("[{}] Parsing CIDR: {}", i, cidr);
                            match cidr.parse::<IpNetwork>() {
                                Ok(ip_net) => {
                                    //info!("Successfully parsed network: {}", ip_net);
                                    ranges.push(IpRange::new(
                                        cidr.to_string(),
                                        source.category,
                                        &source.name,
                                        source.format
                                    ));
                                }
                                Err(e) => {
                                    error!("Failed to parse CIDR '{}': {}", cidr, e);
                                }
                            }
                        } else {
                            error!("Expected string in MISP list at index {}", i);
                        }
                    }
                    //info!("Processed {} networks from MISP warning list", ranges.len());
                    return Ok(ranges);
                }
                // Handle root array format (["cidr1", "cidr2", ...])
                else if let Some(array) = json_value.as_array() {
                    //info!("Found root array format with {} entries", array.len());
                    for (i, cidr_value) in array.iter().enumerate() {
                        if let Some(cidr) = cidr_value.as_str() {
                            //info!("[{}] Parsing CIDR: {}", i, cidr);
                            match cidr.parse::<IpNetwork>() {
                                Ok(ip_net) => {
                                    //info!("Successfully parsed network: {}", ip_net);
                                    ranges.push(IpRange::new(
                                        cidr.to_string(),
                                        source.category,
                                        &source.name,
                                        source.format
                                    ));
                                }
                                Err(e) => {
                                    error!("Failed to parse CIDR '{}': {}", cidr, e);
                                }
                            }
                        } else {
                            error!("Expected string in array at index {}", i);
                        }
                    }
                    //info!("Processed {} networks from root array", ranges.len());
                    return Ok(ranges);
                } else {
                    info!("JSON is not in a recognized format, falling back to text parsing");
                }
            } else {
                info!("Content is not valid JSON, falling back to text parsing");
            }
            
            // If we get here, it's not in MISP format, try parsing as plain text
            info!("Falling back to plain text parsing");
            for (i, line) in content.lines().enumerate() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                
                info!("[{}] Parsing line: {}", i, line);
                match line.parse::<IpNetwork>() {
                    Ok(ip_net) => {
                        info!("Successfully parsed network: {}", ip_net);
                        ranges.push(IpRange::new(
                            ip_net.to_string(),
                            source.category,
                            &source.name,
                            source.format
                        ));
                    }
                    Err(e) => {
                        error!("Failed to parse line '{}' as network: {}", line, e);
                    }
                }
            }
            
            info!("Processed {} networks from plain text", ranges.len());
            return Ok(ranges);
        }
        
        // Handle other formats
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
    
            match source.format {
                SourceFormat::TorExitList => {
                    // Skip non-IP lines in Tor exit node list
                    if line.starts_with("ExitNode") || line.starts_with("Published") || line.starts_with("LastStatus") {
                        continue;
                    }
                    
                    // Format is "ExitAddress IP NICKNAME" - we want the second field
                    if let Some(ip_part) = line.split_whitespace().nth(1) {
                        if let Ok(ip) = ip_part.parse::<IpAddr>() {
                            let network = match ip {
                                IpAddr::V4(_) => format!("{}/32", ip),
                                IpAddr::V6(_) => format!("{}/128", ip),
                            };
                            ranges.push(IpRange::new(network, source.category, &source.name, source.format));
                        } else {
                            error!("Failed to parse IP address at line {}: '{}'", line_num + 1, line);
                        }
                    }
                },
                SourceFormat::IpPort => {
                    // Extract IP from IP:PORT format
                    if let Some(ip_str) = line.split(':').next() {
                        if let Ok(ip) = ip_str.parse::<IpAddr>() {
                            let network = match ip {
                                IpAddr::V4(_) => format!("{}/32", ip_str),
                                IpAddr::V6(_) => format!("{}/128", ip_str),
                            };
                            ranges.push(IpRange::new(network, source.category, &source.name, source.format));
                        } else {
                            error!("Failed to parse IP address at line {}: '{}'", line_num + 1, line);
                        }
                    }
                },
                SourceFormat::Default => {
                    // Default format - try to parse as CIDR, then as plain IP
                    if let Ok(network) = line.parse::<IpNetwork>() {
                        ranges.push(IpRange::new(network.to_string(), source.category, &source.name, source.format));
                    } else if let Ok(ip) = line.parse::<IpAddr>() {
                        let network = match ip {
                            IpAddr::V4(_) => format!("{}/32", ip),
                            IpAddr::V6(_) => format!("{}/128", ip),
                        };
                        ranges.push(IpRange::new(network, source.category, &source.name, source.format));
                    } else {
                        error!("Failed to parse IP network at line {}: '{}'", line_num + 1, line);
                    }
                },
                SourceFormat::JsonList => {
                    // This should never be reached due to the early return above
                    error!("Unexpected JsonList format in line processing loop");
                },
            }
        }
    
        Ok(ranges)
    }

    /// Get the last modified time of a file
    pub fn last_modified(&self, path: &Path) -> Option<DateTime<Utc>> {
        let metadata = fs::metadata(path).ok()?;
        let modified = metadata.modified().ok()?;
        let datetime: DateTime<Utc> = modified.into();
        Some(datetime)
    }

    /// Check if a file needs to be updated
    pub fn needs_update(&self, path: &Path) -> bool {
        match self.last_modified(path) {
            Some(modified) => {
                let now = Utc::now();
                let age = now - modified;
                age.num_seconds() > self.config.max_cache_age_secs as i64
            }
            None => true, // File doesn't exist or can't be read
        }
    }

    /// Generate a filename from a URL, category and IP version
    pub fn filename_from_url(&self, _url: &Url, category: IpCategory, ip_version: IpVersion) -> String {
        // Map category to a simple string representation
        let category_str = match category {
            IpCategory::Vpn => "vpns",
            IpCategory::ProxyHttp => "http_proxies",
            IpCategory::ProxySocks4 => "socks4_proxies",
            IpCategory::ProxySocks5 => "socks5_proxies",
            IpCategory::TorExitNode => "tor_exit_nodes",
            _ => "ranges",
        };
        
        // Add IP version
        match ip_version {
            IpVersion::V4 => format!("{}_v4.txt", category_str),
            IpVersion::V6 => format!("{}_v6.txt", category_str),
        }
    }

    /// Download a file from a URL
    async fn download_file(&self, url: &str) -> Result<String> {
        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .map_err(|e| IpRangeError::IoError(io::Error::new(
                io::ErrorKind::Other,
                e,
            )))?;

        if !response.status().is_success() {
            return Err(IpRangeError::IoError(io::Error::new(
                io::ErrorKind::Other,
                format!("HTTP error: {}", response.status()),
            )));
        }

        response
            .text()
            .await
            .map_err(|e| IpRangeError::IoError(io::Error::new(
                io::ErrorKind::Other,
                e,
            )))
    }
}
