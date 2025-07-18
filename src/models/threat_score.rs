use serde::Serialize;
use std::net::IpAddr;

/// Represents different types of threats that can contribute to the overall threat score
#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub enum ThreatType {
    VpnOrDatacenter,
    Proxy,
    TorExitNode,
    // Add more threat types here as needed
}

/// Represents a single threat finding with its type and weight
#[derive(Debug, Clone, Serialize)]
pub struct ThreatFinding {
    pub threat_type: ThreatType,
    pub description: String,
    pub weight: f32,  // Weight between 0.0 and 1.0 indicating severity
}

/// Configuration for threat scoring
#[derive(Debug, Clone)]
pub struct ThreatScoringConfig {
    pub vpn_weight: f32,
    pub proxy_weight: f32,
    pub tor_weight: f32,
    // Add more weights for future threat types
}

impl Default for ThreatScoringConfig {
    fn default() -> Self {
        Self {
            vpn_weight: 0.6,    // High weight for VPN/Data center
            proxy_weight: 0.8,  // Higher weight for proxies
            tor_weight: 0.9,    // Very high weight for Tor exit nodes
        }
    }
}

/// Calculates a threat score based on various threat findings
#[derive(Debug, Clone, Serialize)]
pub struct ThreatScore {
    pub score: u8,  // 0-100, higher is more suspicious
    pub findings: Vec<ThreatFinding>,
    pub ip: IpAddr,
}

impl ThreatScore {
    /// Creates a new threat score for the given IP address
    pub fn new(ip: IpAddr) -> Self {
        Self {
            score: 0,
            findings: Vec::new(),
            ip,
        }
    }

    /// Adds a threat finding and updates the score
    /*pub fn add_finding(&mut self, finding: ThreatFinding) {
        self.findings.push(finding);
        self.calculate_score();
    }*/

    /// Adds multiple threat findings and updates the score
    pub fn add_findings(&mut self, findings: impl IntoIterator<Item = ThreatFinding>) {
        self.findings.extend(findings);
        self.calculate_score();
    }

    /// Calculates the overall threat score based on all findings
    fn calculate_score(&mut self) {
        let config = ThreatScoringConfig::default();
        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for finding in &self.findings {
            let weight = match finding.threat_type {
                ThreatType::VpnOrDatacenter => config.vpn_weight,
                ThreatType::Proxy => config.proxy_weight,
                ThreatType::TorExitNode => config.tor_weight,
                // Add new threat types here
            };
            
            weighted_sum += finding.weight * weight;
            total_weight += weight;
        }

        // Normalize the score to 0-100 range
        let normalized_score = if total_weight > 0.0 {
            (weighted_sum / total_weight * 100.0) as u8
        } else {
            0
        };

        self.score = normalized_score.min(100); // Cap at 100
    }

    /// Creates a threat score from common IP information
    pub fn from_ip_info(
        ip: IpAddr,
        is_vpn: bool,
        is_proxy: bool,
        proxy_type: Option<&'static str>,
        is_tor: bool,
    ) -> Self {
        let mut score = Self::new(ip);
        let mut findings = Vec::new();

        if is_vpn {
            findings.push(ThreatFinding {
                threat_type: ThreatType::VpnOrDatacenter,
                description: "IP is associated with a VPN or data center".to_string(),
                weight: 1.0,  // Full weight for binary detection
            });
        }

        if is_proxy {
            let proxy_desc = if let Some(proxy_type) = proxy_type {
                format!("IP is a known {} proxy", proxy_type)
            } else {
                "IP is a known proxy".to_string()
            };
            
            findings.push(ThreatFinding {
                threat_type: ThreatType::Proxy,
                description: proxy_desc,
                weight: 1.0,  // Full weight for binary detection
            });
        }

        if is_tor {
            findings.push(ThreatFinding {
                threat_type: ThreatType::TorExitNode,
                description: "IP is a known Tor exit node".to_string(),
                weight: 1.0,  // Full weight for binary detection
            });
        }

        score.add_findings(findings);
        score
    }
}
