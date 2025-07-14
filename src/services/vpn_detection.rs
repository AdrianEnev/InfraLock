use crate::config::Settings;
use ipnetwork::IpNetwork;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{self, BufRead};
use std::net::IpAddr;
use std::path::Path;
use tracing::{debug, info, warn};

static VPN_DETECTOR: Lazy<VpnDetector> = Lazy::new(|| {
    let settings = Settings::new().expect("Failed to load settings");
    VpnDetector::new(&settings).expect("Failed to initialize VpnDetector")
});

/// Detects if an IP address belongs to a known VPN or datacenter network.
pub struct VpnDetector {
    networks: Vec<IpNetwork>,
}

impl VpnDetector {
    /// Creates a new VpnDetector instance and loads networks from the configured path.
    pub fn new(settings: &Settings) -> io::Result<Self> {
        let db_path = settings.resolve_vpn_detector_db_path()?;
        info!("Loading VPN detection database from: {}", db_path.display());
        let networks = Self::load_networks(&db_path)?;
        info!("Loaded {} VPN/datacenter networks", networks.len());
        Ok(Self { networks })
    }

    fn load_networks<P: AsRef<Path>>(path: P) -> io::Result<Vec<IpNetwork>> {
        debug!("Loading networks from: {}", path.as_ref().display());
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        let mut networks = Vec::new();
        let mut line_count = 0;
        let mut invalid_count = 0;

        for line in reader.lines() {
            line_count += 1;
            let line = line?;
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            match line.parse::<IpNetwork>() {
                Ok(network) => networks.push(network),
                Err(e) => {
                    invalid_count += 1;
                    debug!("Failed to parse network '{}': {}", line, e);
                }
            }
        }

        // Sort networks by prefix length (most specific first) for faster lookups
        networks.sort_by(|a, b| b.prefix().cmp(&a.prefix()));
        
        if invalid_count > 0 {
            warn!("Failed to parse {}/{} network entries", invalid_count, line_count);
        }
        debug!("Successfully loaded {} networks ({} invalid entries)", networks.len(), invalid_count);
        
        Ok(networks)
    }

    /// Checks if the given IP address belongs to a known VPN or datacenter network.
    pub fn is_vpn_or_datacenter(&self, ip: IpAddr) -> bool {
        self.networks.iter().any(|network| network.contains(ip))
    }
    
    /// Checks if any IP in the given network range belongs to a known VPN or datacenter network.
    pub fn is_range_vpn_or_datacenter(&self, cidr: &str) -> Option<bool> {
        let input_network = match cidr.parse::<IpNetwork>() {
            Ok(net) => net,
            Err(_) => {
                warn!("Failed to parse network: {}", cidr);
                return None;
            }
        };

        debug!("Checking network: {}", input_network);
        
        // 1. Check if the input network is exactly in our database
        if self.networks.contains(&input_network) {
            debug!("Exact match found in database");
            return Some(true);
        }
    
        // 2. Check for any overlap with our networks
        for (i, vpn_net) in self.networks.iter().enumerate() {
            if vpn_net.prefix() <= input_network.prefix() && vpn_net.contains(input_network.ip()) ||  // Partial overlap
               input_network.prefix() <= vpn_net.prefix() && input_network.contains(vpn_net.ip())     // Partial overlap
            {
                debug!("Overlap found with VPN network #{}: {}", i, vpn_net);
                return Some(true);
            }
        }

        // 3. For small networks, do a full scan
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
                if self.is_vpn_or_datacenter(ip) {
                    debug!("Found VPN IP in range: {}", ip);
                    return Some(true);
                }
            }
        }
    
        debug!("No VPN found in network {}", input_network);
        Some(false)
    }
    
    /// Returns a reference to the global VpnDetector instance.
    pub fn get() -> &'static VpnDetector {
        &VPN_DETECTOR
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_performance() {
        let detector = VpnDetector::get();
        let start = Instant::now();
        detector.is_vpn_or_datacenter("1.1.1.1".parse().unwrap());
        let duration = start.elapsed();
        
        debug!("Lookup took: {:?}", duration);
        assert!(duration < std::time::Duration::from_millis(10), "Lookup took too long");
    }

    #[test]
    fn test_vpn_detection() {
        let detector = VpnDetector::get();
        
        // Test cases: (input, expected_result)
        let test_cases = [
            // Known VPN IPs (replace with actual test cases)
            ("1.1.1.1", false),
            ("8.8.8.8", false),
        ];
        
        for (input, expected) in test_cases.iter() {
            info!("Testing VPN detection for: {}", input);
            
            if let Ok(ip) = input.parse::<IpAddr>() {
                // Test single IP
                let result = detector.is_vpn_or_datacenter(ip);
                info!("IP check - Result: {}, Expected: {}", result, expected);
                if ip.to_string() == *input {  // Only assert if it was a single IP
                    assert_eq!(result, *expected, "Mismatch for IP: {}", input);
                }
            }
            
            // Test network range
            if let Ok(network) = input.parse::<IpNetwork>() {
                debug!("Network: {}", network);
                debug!("Network prefix: {}", network.prefix());
                debug!(
                    "Network size: {} IPs",
                    2u32.pow(32 - network.prefix() as u32)
                );
                
                // Check if the network is in our database
                let is_in_db = detector.networks.contains(&network);
                debug!("Exact network in database: {}", is_in_db);
                
                // Check if any IP in the network is in our database
                let result = detector.is_range_vpn_or_datacenter(input);
                info!("Network range check - Result: {:?}, Expected: {}", result, expected);
                
                if let Some(found) = result {
                    assert_eq!(found, *expected, "Mismatch for network: {}", input);
                } else {
                    panic!("Failed to check network: {}", input);
                }
            } else {
                warn!("Not a valid network: {}", input);
            }
        }
    }
}