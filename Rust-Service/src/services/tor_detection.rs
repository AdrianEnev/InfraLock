use crate::config::Settings;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{self, BufRead};
use std::net::IpAddr;
use std::path::Path;
use std::collections::HashSet;
use tracing::{debug, info, warn};

static TOR_DETECTOR: Lazy<TorDetector> = Lazy::new(|| {
    let settings = Settings::new().expect("Failed to load settings");
    TorDetector::new(&settings).expect("Failed to initialize TorDetector")
});

/// Detects if an IP address is a known Tor exit node.
pub struct TorDetector {
    exit_nodes: HashSet<IpAddr>,
}

impl TorDetector {
    /// Creates a new TorDetector instance and loads exit node IPs from the configured path.
    pub fn new(settings: &Settings) -> io::Result<Self> {
        let db_path = settings.resolve_tor_detector_db_path()?;
        info!("Loading Tor exit nodes from: {}", db_path.display());
        let exit_nodes = Self::load_exit_nodes(&db_path)?;
        info!("Loaded {} Tor exit nodes", exit_nodes.len());
        
        Ok(Self { exit_nodes })
    }

    fn load_exit_nodes<P: AsRef<Path>>(path: P) -> io::Result<HashSet<IpAddr>> {
        debug!("Loading Tor exit nodes from: {}", path.as_ref().display());
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        let mut exit_nodes = HashSet::new();
        let mut line_count = 0;
        let mut invalid_count = 0;
        let mut duplicate_count = 0;

        for line in reader.lines() {
            line_count += 1;
            let line = line?;
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // The exit-addresses.txt file typically has format: "ExitNode $FINGERPRINT"
            // followed by "ExitAddress $IP $DATE"
            if line.starts_with("ExitAddress ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    match parts[1].parse::<IpAddr>() {
                        Ok(ip) => {
                            if !exit_nodes.insert(ip) {
                                duplicate_count += 1;
                            }
                        },
                        Err(e) => {
                            invalid_count += 1;
                            debug!("Failed to parse Tor exit node IP '{}': {}", parts[1], e);
                        }
                    }
                } else {
                    invalid_count += 1;
                    debug!("Invalid Tor exit node entry format: {}", line);
                }
            }
        }

        if invalid_count > 0 {
            warn!("Failed to parse {}/{} Tor exit node entries", invalid_count, line_count);
        }
        if duplicate_count > 0 {
            debug!("Skipped {} duplicate Tor exit node IPs", duplicate_count);
        }
        debug!("Successfully loaded {} unique Tor exit nodes ({} invalid entries)", 
               exit_nodes.len(), invalid_count);
        
        Ok(exit_nodes)
    }

    /// Checks if the given IP address is a known Tor exit node.
    pub fn is_tor_exit_node(&self, ip: IpAddr) -> bool {
        self.exit_nodes.contains(&ip)
    }
    
    /// Returns a reference to the global TorDetector instance.
    pub fn get() -> &'static Self {
        &TOR_DETECTOR
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};
    use tempfile::NamedTempFile;
    use std::io::Write;

    fn create_test_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "{}", content).unwrap();
        file
    }

    #[test]
    fn test_load_exit_nodes() {
        let content = r"# This is a comment
            ExitNode ABCDEF1234567890ABCDEF1234567890ABCDEF12
            ExitAddress 1.2.3.4 2023-01-01 12:00:00
            ExitNode FEDCBA0987654321FEDCBA0987654321FEDCBA09
            ExitAddress 5.6.7.8 2023-01-01 12:00:00
            ExitAddress 2001:db8::1 2023-01-01 12:00:00
            # Another comment
            ExitAddress 1.2.3.4 2023-01-02 12:00:00  # Duplicate
            ExitAddress invalid-ip 2023-01-01 12:00:00";
        
        let file = create_test_file(content);
        let exit_nodes = TorDetector::load_exit_nodes(file.path()).unwrap();
        
        assert!(exit_nodes.contains(&IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))));
        assert!(exit_nodes.contains(&IpAddr::V4(Ipv4Addr::new(5, 6, 7, 8))));
        assert!(exit_nodes.contains(&IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1))));
        assert_eq!(exit_nodes.len(), 3); // Should have 3 unique IPs (1.2.3.4, 5.6.7.8, 2001:db8::1)
    }

    #[test]
    fn test_is_tor_exit_node() {
        let content = "ExitAddress 1.2.3.4 2023-01-01 12:00:00\nExitAddress 2001:db8::1 2023-01-01 12:00:00";
        let file = create_test_file(content);
        
        let detector = TorDetector {
            exit_nodes: TorDetector::load_exit_nodes(file.path()).unwrap(),
        };
        
        assert!(detector.is_tor_exit_node(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))));
        assert!(detector.is_tor_exit_node(IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1))));
        assert!(!detector.is_tor_exit_node(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))));
    }
}
