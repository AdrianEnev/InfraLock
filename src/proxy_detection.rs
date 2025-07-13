use crate::config::Settings;
use ipnetwork::IpNetwork;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{self, BufRead};
use std::net::IpAddr;
use std::path::{Path};
use std::collections::HashSet;
use tracing::{debug, info, warn};

static PROXY_DETECTOR: Lazy<ProxyDetector> = Lazy::new(|| {
    let settings = Settings::new().expect("Failed to load settings");
    ProxyDetector::new(&settings).expect("Failed to initialize ProxyDetector")
});

/// Detects if an IP address is a known proxy server.
pub struct ProxyDetector {
    http_proxies: HashSet<IpAddr>,
    socks4_proxies: HashSet<IpAddr>,
    socks5_proxies: HashSet<IpAddr>,
}

impl ProxyDetector {
    /// Creates a new ProxyDetector instance and loads proxy IPs from the configured paths.
    pub fn new(settings: &Settings) -> io::Result<Self> {
        let (http_path, socks4_path, socks5_path) = settings.resolve_proxy_detector_db_paths()?;
        
        info!("Loading HTTP proxies from: {}", http_path.display());
        let http_proxies = Self::load_proxy_ips(&http_path)?;
        info!("Loaded {} HTTP proxies", http_proxies.len());
        
        info!("Loading SOCKS4 proxies from: {}", socks4_path.display());
        let socks4_proxies = Self::load_proxy_ips(&socks4_path)?;
        info!("Loaded {} SOCKS4 proxies", socks4_proxies.len());
        
        info!("Loading SOCKS5 proxies from: {}", socks5_path.display());
        let socks5_proxies = Self::load_proxy_ips(&socks5_path)?;
        info!("Loaded {} SOCKS5 proxies", socks5_proxies.len());
        
        Ok(Self {
            http_proxies,
            socks4_proxies,
            socks5_proxies,
        })
    }

    fn load_proxy_ips<P: AsRef<Path>>(path: P) -> io::Result<HashSet<IpAddr>> {
        debug!("Loading proxy IPs from: {}", path.as_ref().display());
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        let mut proxy_ips = HashSet::new();
        let mut line_count = 0;
        let mut invalid_count = 0;
        let mut duplicate_count = 0;

        for line in reader.lines() {
            line_count += 1;
            let line = line?;
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Extract just the IP part (before the port)
            if let Some(ip_part) = line.split(':').next() {
                match ip_part.parse::<IpAddr>() {
                    Ok(ip) => {
                        if !proxy_ips.insert(ip) {
                            duplicate_count += 1;
                        }
                    },
                    Err(e) => {
                        invalid_count += 1;
                        debug!("Failed to parse proxy IP '{}': {}", ip_part, e);
                    }
                }
            } else {
                invalid_count += 1;
                debug!("Invalid proxy entry format: {}", line);
            }
        }

        if invalid_count > 0 {
            warn!("Failed to parse {}/{} proxy entries", invalid_count, line_count);
        }
        if duplicate_count > 0 {
            debug!("Skipped {} duplicate proxy IPs", duplicate_count);
        }
        debug!("Successfully loaded {} unique proxy IPs ({} invalid entries)", 
               proxy_ips.len(), invalid_count);
        
        Ok(proxy_ips)
    }

    /// Checks if the given IP address is a known proxy and returns its type if found.
    /// Returns None if the IP is not a known proxy.
    pub fn check_proxy(&self, ip: IpAddr) -> Option<&'static str> {
        if self.http_proxies.contains(&ip) {
            Some("HTTP/HTTPS")
        } else if self.socks5_proxies.contains(&ip) {
            Some("SOCKS5")
        } else if self.socks4_proxies.contains(&ip) {
            Some("SOCKS4")
        } else {
            None
        }
    }

    /// Checks if the given IP address is a known proxy server.
    /// Returns true if the IP is any type of proxy.
    pub fn is_proxy(&self, ip: IpAddr) -> bool {
        self.check_proxy(ip).is_some()
    }

    /// Checks if any IP in the given network range is a known proxy server.
    pub fn is_range_proxy(&self, cidr: &str) -> Option<bool> {
        let input_network = match cidr.parse::<IpNetwork>() {
            Ok(net) => net,
            Err(_) => {
                warn!("Failed to parse network: {}", cidr);
                return None;
            }
        };

        debug!("Checking network for proxy IPs: {}", input_network);
        
        // For small networks, do a full scan
        let prefix = input_network.prefix();
        if prefix >= 24 {  // For /24 and larger networks (256 IPs or fewer)
            let ip_count = 2u32.pow(32 - prefix as u32);
            info!(
                "Performing full IP scan for {}/{} ({} IPs)",
                input_network.ip(),
                prefix,
                ip_count
            );
            
            for ip in input_network.iter() {
                if self.is_proxy(ip) {
                    debug!("Found proxy IP in range: {}", ip);
                    return Some(true);
                }
            }
            debug!("No proxy IPs found in network {}", input_network);
            return Some(false);
        }

        // For larger networks, just check the network IP
        debug!("Checking network IP for proxy: {}", input_network.ip());
        Some(self.is_proxy(input_network.ip()))
    }
    
    /// Returns a reference to the global ProxyDetector instance.
    pub fn get() -> &'static ProxyDetector {
        &PROXY_DETECTOR
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::fs;
    use tempfile;

    fn create_test_settings() -> (Settings, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        
        // Create HTTP proxies test file
        let http_path = dir.path().join("http.txt");
        let mut http_file = fs::File::create(&http_path).unwrap();
        writeln!(http_file, "1.1.1.1:8080").unwrap();
        writeln!(http_file, "2.2.2.2:3128").unwrap();
        
        // Create SOCKS4 proxies test file
        let socks4_path = dir.path().join("socks4.txt");
        let mut socks4_file = fs::File::create(&socks4_path).unwrap();
        writeln!(socks4_file, "3.3.3.3:1080").unwrap();
        
        // Create SOCKS5 proxies test file
        let socks5_path = dir.path().join("socks5.txt");
        let mut socks5_file = fs::File::create(&socks5_path).unwrap();
        writeln!(socks5_file, "4.4.4.4:1080").unwrap();

        let mut settings = Settings::default();
        settings.proxy_detector.http_db_path = http_path;
        settings.proxy_detector.socks4_db_path = socks4_path;
        settings.proxy_detector.socks5_db_path = socks5_path;
        
        (settings, dir)
    }

    #[test]
    fn test_proxy_detection() {
        let (settings, _dir) = create_test_settings();
        let detector = ProxyDetector::new(&settings).unwrap();
        
        // Test HTTP proxy detection
        assert_eq!(detector.check_proxy("1.1.1.1".parse().unwrap()), Some("HTTP/HTTPS"));
        assert_eq!(detector.check_proxy("2.2.2.2".parse().unwrap()), Some("HTTP/HTTPS"));
        
        // Test SOCKS4 proxy detection
        assert_eq!(detector.check_proxy("3.3.3.3".parse().unwrap()), Some("SOCKS4"));
        
        // Test SOCKS5 proxy detection
        assert_eq!(detector.check_proxy("4.4.4.4".parse().unwrap()), Some("SOCKS5"));
        
        // Test non-proxy IP
        assert_eq!(detector.check_proxy("8.8.8.8".parse().unwrap()), None);
        
        // Test is_proxy (any type)
        assert!(detector.is_proxy("1.1.1.1".parse().unwrap()));
        assert!(!detector.is_proxy("8.8.8.8".parse().unwrap()));
    }

    #[test]
    fn test_range_proxy_detection() {
        let (settings, _dir) = create_test_settings();
        let detector = ProxyDetector::new(&settings).unwrap();
        
        // Test range containing HTTP proxy
        assert_eq!(detector.is_range_proxy("1.1.1.0/24"), Some(true));
        
        // Test range containing SOCKS4 proxy
        assert_eq!(detector.is_range_proxy("3.3.3.0/24"), Some(true));
        
        // Test range containing SOCKS5 proxy
        assert_eq!(detector.is_range_proxy("4.4.4.0/24"), Some(true));
        
        // Test range with no proxies
        assert_eq!(detector.is_range_proxy("8.8.8.0/24"), Some(false));
        
        // Test invalid range
        assert_eq!(detector.is_range_proxy("invalid"), None);
    }
}