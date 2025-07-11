use crate::config::Settings;
use ipnetwork::IpNetwork;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{self, BufRead};
use std::net::IpAddr;
use std::path::Path;

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
        let networks = Self::load_networks(&db_path)?;
        Ok(Self { networks })
    }

    fn load_networks<P: AsRef<Path>>(path: P) -> io::Result<Vec<IpNetwork>> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        let mut networks = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Ok(network) = line.parse::<IpNetwork>() {
                networks.push(network);
            }
        }

        // Sort networks by prefix length (most specific first) for faster lookups
        networks.sort_by(|a, b| b.prefix().cmp(&a.prefix()));
        Ok(networks)
    }

    /// Checks if the given IP address belongs to a known VPN or datacenter network.
    pub fn is_vpn_or_datacenter(&self, ip: IpAddr) -> bool {
        self.networks.iter().any(|network| network.contains(ip))
    }
    
    /// Checks if any IP in the given network range belongs to a known VPN or datacenter network.
    /// Returns `Some(true)` if any IP in the range is a VPN/datacenter IP,
    /// `Some(false)` if no IPs in the range are VPN/datacenter IPs,
    /// or `None` if the input is not a valid network range.
    pub fn is_range_vpn_or_datacenter(&self, cidr: &str) -> Option<bool> {
        // Try to parse as a network range
        let input_network = match cidr.parse::<IpNetwork>() {
            Ok(net) => net,
            Err(_) => {
                println!("Failed to parse network: {}", cidr);
                return None;
            }
        };
    
        println!("Checking network: {}", input_network);
        
        // 1. Check if the input network is exactly in our database
        if self.networks.contains(&input_network) {
            println!("Exact match found in database");
            return Some(true);
        }
    
        // 2. Check if any of our VPN networks overlap with the input network
        for (i, vpn_net) in self.networks.iter().enumerate() {
            if vpn_net.contains(input_network.ip()) ||  // Input IP is inside a VPN network
               input_network.contains(vpn_net.ip()) ||  // VPN network IP is inside input range
               vpn_net.prefix() <= input_network.prefix() && vpn_net.contains(input_network.ip()) ||  // Partial overlap
               input_network.prefix() <= vpn_net.prefix() && input_network.contains(vpn_net.ip())     // Partial overlap
            {
                println!("Overlap found with VPN network #{}: {}", i, vpn_net);
                return Some(true);
            }
        }
    
        // 3. For small networks, do a full scan
        let prefix = input_network.prefix();
        if prefix >= 24 {  // For /24 and larger networks (256 IPs or fewer)
            println!("Doing full IP scan for /{0} network (size: {1} IPs)", prefix, 2u32.pow(32 - prefix as u32));
            for ip in input_network.iter() {
                if self.is_vpn_or_datacenter(ip) {
                    println!("Found VPN IP in range: {}", ip);
                    return Some(true);
                }
            }
        }
    
        println!("No VPN found in network");
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
    use std::net::IpAddr;
    use ipnetwork::IpNetwork;

    #[test]
    fn test_performance() {
        let settings = Settings::new().unwrap();
        let detector = VpnDetector::new(&settings).unwrap();
        
        // Time the lookup
        let start = std::time::Instant::now();
        detector.is_vpn_or_datacenter("1.1.1.1".parse().unwrap());
        let duration = start.elapsed();
        
        println!("Lookup took: {:?}", duration);
        assert!(duration < std::time::Duration::from_millis(10), "Lookup took too long");
    }

    #[test]
    fn test_vpn_detection() {
        let settings = Settings::new().unwrap();
        let detector = VpnDetector::new(&settings).unwrap();
        
        // Test with a known VPN range from ipv4.txt
        let test_cases = [
            ("221.121.128.0/19", true),  // This should be in the database
            ("1.1.1.1", false),          // This should not be a VPN
            ("220.158.32.0/23", true),   // This should be in the database
        ];
        
        for (input, expected) in test_cases.iter() {
            println!("\nTesting: {}", input);
            
            if let Ok(ip) = input.parse::<IpAddr>() {
                // Test single IP
                let result = detector.is_vpn_or_datacenter(ip);
                println!("Single IP check - Result: {}, Expected: {}", result, expected);
                if ip.to_string() == *input {  // Only assert if it was a single IP
                    assert_eq!(result, *expected, "Mismatch for IP: {}", input);
                }
            }
            
            // Test network range
            if let Ok(network) = input.parse::<IpNetwork>() {
                println!("Network: {}", network);
                println!("Network prefix: {}", network.prefix());
                println!("Network size: {} IPs", 2u32.pow(32 - network.prefix() as u32));
                
                // Check if the network is in our database
                let is_in_db = detector.networks.contains(&network);
                println!("Exact network in database: {}", is_in_db);
                
                // Check if any IP in the network is in our database
                let result = detector.is_range_vpn_or_datacenter(input);
                println!("Range check - Result: {:?}, Expected: {}", result, expected);
                
                if let Some(found) = result {
                    assert_eq!(found, *expected, "Mismatch for network: {}", input);
                } else {
                    panic!("Failed to check network: {}", input);
                }
            } else {
                println!("Not a valid network: {}", input);
            }
        }
    }
}