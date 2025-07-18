use serde::Serialize;
use crate::models::threat_score::{ThreatScore, ThreatType};

/// Represents the recommended response action for a given threat level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResponseAction {
    /// Allow the request without any challenges
    Allow,
    
    /// Monitor the request but allow it to proceed
    Monitor,
    
    /// Present a challenge to the user (e.g., CAPTCHA, 2FA)
    Challenge,
    
    /// Redirect the user to a different page (e.g., rate limit page, maintenance page)
    Redirect,
    
    /// Block the request entirely
    Block,
}

/// Configuration for response action determination
#[derive(Debug, Clone)]
pub struct ResponseActionConfig {
    /// Threshold for Monitor action (0-100)
    pub monitor_threshold: u8,
    
    /// Threshold for Challenge action (0-100)
    pub challenge_threshold: u8,
    
    /// Threshold for Redirect action (0-100)
    pub redirect_threshold: u8,
    
    /// List of threat types that should always trigger a block
    pub block_immediate: Vec<ThreatType>,
    
    /// Whether to enable monitoring mode (logs but doesn't block)
    pub monitor_mode: bool,
}

impl Default for ResponseActionConfig {
    fn default() -> Self {
        Self {
            monitor_threshold: 20,    // 0-20: Allow, 21-50: Monitor
            challenge_threshold: 50,  // 51-75: Challenge
            redirect_threshold: 75,   // 76-100: Redirect
            block_immediate: vec![
                ThreatType::TorExitNode,  // Always block Tor exit nodes
            ],
            monitor_mode: false,
        }
    }
}

/// Service for determining the appropriate response action based on threat assessment
pub struct ResponseActionService {
    config: ResponseActionConfig,
}

impl ResponseActionService {
    /// Creates a new ResponseActionService with default configuration
    pub fn new() -> Self {
        Self::with_config(ResponseActionConfig::default())
    }
    
    /// Creates a new ResponseActionService with custom configuration
    pub fn with_config(config: ResponseActionConfig) -> Self {
        Self { config }
    }
    
    /// Determines the recommended response action based on the threat score and findings
    /// Takes in already-implemented ThreatScore struct
    pub fn determine_action(&self, threat_score: &ThreatScore) -> ResponseAction {
        // First check for immediate blocks
        for finding in &threat_score.findings {
            if self.config.block_immediate.contains(&finding.threat_type) {
                return if self.config.monitor_mode {
                    ResponseAction::Monitor
                } else {
                    ResponseAction::Block
                };
            }
        }
        
        // If in monitor mode, just monitor regardless of score
        if self.config.monitor_mode {
            return ResponseAction::Monitor;
        }
        
        // Determine action based on score thresholds
        let score = threat_score.score;
        
        if score > self.config.redirect_threshold {
            ResponseAction::Redirect
        } else if score > self.config.challenge_threshold {
            ResponseAction::Challenge
        } else if score > self.config.monitor_threshold {
            ResponseAction::Monitor
        } else {
            ResponseAction::Allow
        }
    }
    
    // Convenience method to determine action from raw score and findings
    // Takes in raw score and findings, and creates a ThreatScore struct
    /*pub fn determine_action_from_score(
        &self, 
        ip: IpAddr,
        score: u8, 
        findings: Vec<(ThreatType, String)>
    ) -> ResponseAction {
        let threat_score = ThreatScore {
            score,
            findings: findings.into_iter()
                .map(|(threat_type, description)| crate::models::threat_score::ThreatFinding {
                    threat_type,
                    description,
                    weight: 1.0,
                })
                .collect(),
            ip,
        };
        
        self.determine_action(&threat_score)
    }*/
}

impl Default for ResponseActionService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;
    use std::net::IpAddr;
    
    #[test]
    fn test_determine_action() {
        let service = ResponseActionService::new();
        let ip = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
        
        // Test score ranges
        let test_cases = vec![
            (0, ResponseAction::Allow),
            (10, ResponseAction::Allow),
            (20, ResponseAction::Allow),
            (21, ResponseAction::Monitor),
            (35, ResponseAction::Monitor),
            (50, ResponseAction::Monitor),
            (51, ResponseAction::Challenge),
            (60, ResponseAction::Challenge),
            (75, ResponseAction::Challenge),
            (76, ResponseAction::Redirect),
            (90, ResponseAction::Redirect),
            (100, ResponseAction::Redirect),
        ];
        
        for (score, expected) in test_cases {
            let test_score = ThreatScore {
                score,
                findings: vec![],
                ip,
            };
            assert_eq!(
                service.determine_action(&test_score),
                expected,
                "Failed for score: {}",
                score
            );
        }
        
        // Test immediate block for Tor exit nodes
        let tor_score = ThreatScore {
            score: 10,  // Low score but should be blocked immediately
            findings: vec![crate::models::threat_score::ThreatFinding {
                threat_type: ThreatType::TorExitNode,
                description: "Tor exit node".to_string(),
                weight: 1.0,
            }],
            ip,
        };
        assert_eq!(service.determine_action(&tor_score), ResponseAction::Block);
        
        // Test monitor mode
        let monitor_service = ResponseActionService::with_config(ResponseActionConfig {
            monitor_mode: true,
            ..Default::default()
        });
        
        let high_score = ThreatScore {
            score: 100,
            findings: vec![],
            ip,
        };
        
        assert_eq!(monitor_service.determine_action(&high_score), ResponseAction::Monitor);
        assert_eq!(monitor_service.determine_action(&tor_score), ResponseAction::Monitor);
    }
}
