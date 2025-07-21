use lazy_static::lazy_static;
use prometheus::{register_int_counter, IntCounter};
use std::sync::Mutex;
use std::time::{Duration};
use tokio::time;

lazy_static! {
    // Track consecutive failures for alerting
    static ref CONSECUTIVE_FAILURES: Mutex<u32> = Mutex::new(0);
    static ref ALERT_STATE: Mutex<AlertState> = Mutex::new(AlertState::Normal);
    static ref ALERT_COUNTER: IntCounter = register_int_counter!(
        "api_key_validation_alert_count",
        "Total number of times alerts have been triggered"
    ).unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AlertState {
    Normal,
    Pending,
    Firing,
}

/// Configuration for alerting thresholds
#[derive(Clone, Debug)]
pub struct AlertConfig {
    /// Number of consecutive failures before alerting
    pub failure_threshold: u32,
    /// Duration to wait before sending an alert
    pub pending_duration: Duration,
    /// Cooldown period after an alert is resolved
    pub cooldown_duration: Duration,
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            pending_duration: Duration::from_secs(300),  // 5 minutes
            cooldown_duration: Duration::from_secs(1800), // 30 minutes
        }
    }
}

/// Record a successful API key validation
pub fn record_success() {
    let mut failures = CONSECUTIVE_FAILURES.lock().unwrap();
    let mut alert_state = ALERT_STATE.lock().unwrap();
    
    *failures = 0;
    
    // If we were in a failure state, reset to normal
    if *alert_state != AlertState::Normal {
        *alert_state = AlertState::Normal;
        tracing::info!("API key validation recovered to normal state");
    }
}

/// Record a failed API key validation
pub fn record_failure(error_type: &str) {
    let mut failures = CONSECUTIVE_FAILURES.lock().unwrap();
    *failures += 1;
    
    let mut alert_state = ALERT_STATE.lock().unwrap();
    let config = AlertConfig::default();
    
    match *alert_state {
        AlertState::Normal => {
            if *failures >= config.failure_threshold {
                *alert_state = AlertState::Pending;
                tracing::warn!(
                    count = *failures,
                    "API key validation failure threshold reached, entering pending state"
                );
                
                // Start a background task to handle the pending alert
                let error_type = error_type.to_string();
                tokio::spawn(async move {
                    time::sleep(config.pending_duration).await;
                    check_and_alert(error_type).await;
                });
            } else {
                tracing::warn!(
                    count = *failures,
                    threshold = config.failure_threshold,
                    "API key validation failed"
                );
            }
        }
        AlertState::Pending => {
            // Already have an alert pending, just log
            tracing::warn!(
                count = *failures,
                "API key validation still failing, alert pending"
            );
        }
        AlertState::Firing => {
            // Already in firing state, just log
            tracing::error!(
                count = *failures,
                "API key validation still failing, alert is firing"
            );
        }
    }
}

/// Check if we should alert and trigger the alert if needed
async fn check_and_alert(error_type: String) {
    let mut alert_state = ALERT_STATE.lock().unwrap();
    let failures = *CONSECUTIVE_FAILURES.lock().unwrap();
    let config = AlertConfig::default();
    
    if failures >= config.failure_threshold && *alert_state != AlertState::Firing {
        // Only fire the alert if we're not already in firing state
        *alert_state = AlertState::Firing;
        ALERT_COUNTER.inc();
        
        // In a real implementation, this would trigger an actual alert (e.g., PagerDuty, OpsGenie, etc.)
        tracing::error!(
            error_type,
            count = failures,
            "ALERT: API key validation is failing repeatedly"
        );
        
        // Set up a cooldown period before we can alert again
        let alert_state = *alert_state; // Get the current state value
        tokio::spawn(async move {
            time::sleep(config.cooldown_duration).await;
            let mut alert_state_guard = ALERT_STATE.lock().unwrap();
            if *alert_state_guard == AlertState::Firing && alert_state == AlertState::Firing {
                *alert_state_guard = AlertState::Normal;
                tracing::info!("Alert cooldown period ended, resetting to normal state");
            }
        });
    } else if *alert_state == AlertState::Pending {
        // If we're in pending state but failures have been reset, go back to normal
        *alert_state = AlertState::Normal;
    }
}

/// Initialize the alerting system
pub fn init() {
    // In a real implementation, this would set up any required alerting integrations
    tracing::info!("Alerting system initialized");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_alerting_flow() {
        // Reset state
        *CONSECUTIVE_FAILURES.lock().unwrap() = 0;
        *ALERT_STATE.lock().unwrap() = AlertState::Normal;
        
        // Test normal operation
        record_success();
        assert_eq!(*CONSECUTIVE_FAILURES.lock().unwrap(), 0);
        assert_eq!(*ALERT_STATE.lock().unwrap(), AlertState::Normal);
        
        // Test failure threshold
        let config = AlertConfig::default();
        for i in 1..=config.failure_threshold {
            record_failure("test_error");
            assert_eq!(*CONSECUTIVE_FAILURES.lock().unwrap(), i);
        }
        
        // Should be in pending state now
        assert_eq!(*ALERT_STATE.lock().unwrap(), AlertState::Pending);
        
        // Wait for pending duration plus a small buffer
        tokio::time::sleep(config.pending_duration + Duration::from_secs(1)).await;
        
        // Should be in firing state now
        assert_eq!(*ALERT_STATE.lock().unwrap(), AlertState::Firing);
        
        // Test recovery
        record_success();
        assert_eq!(*ALERT_STATE.lock().unwrap(), AlertState::Normal);
    }
}
