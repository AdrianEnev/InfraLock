use lazy_static::lazy_static;
use prometheus::{self, Encoder, Histogram, IntCounter, IntCounterVec, register_int_counter, register_int_counter_vec, register_histogram};

lazy_static! {
    // API Key Validation Metrics
    pub static ref API_KEY_VALIDATION_TOTAL: IntCounter = register_int_counter!(
        "api_key_validation_total",
        "Total number of API key validation attempts"
    ).unwrap();

    pub static ref API_KEY_VALIDATION_SUCCESS: IntCounter = register_int_counter!(
        "api_key_validation_success_total",
        "Total number of successful API key validations"
    ).unwrap();

    pub static ref API_KEY_VALIDATION_FAILED: IntCounterVec = register_int_counter_vec!(
        "api_key_validation_failed_total",
        "Total number of failed API key validations by reason",
        &["reason"]
    ).unwrap();

    pub static ref API_KEY_VALIDATION_DURATION: Histogram = register_histogram!(
        "api_key_validation_duration_seconds",
        "The duration of API key validation requests in seconds"
    ).unwrap();

    // Circuit Breaker Metrics
    pub static ref CIRCUIT_BREAKER_STATE: IntCounterVec = register_int_counter_vec!(
        "circuit_breaker_state_changes_total",
        "Total number of circuit breaker state changes",
        &["state"]
    ).unwrap();

    pub static ref CIRCUIT_BREAKER_REJECTED: IntCounter = register_int_counter!(
        "circuit_breaker_rejected_requests_total",
        "Total number of requests rejected by circuit breaker"
    ).unwrap();

    // Cache Metrics
    pub static ref CACHE_HITS: IntCounter = register_int_counter!(
        "cache_hits_total",
        "Total number of cache hits"
    ).unwrap();

    pub static ref CACHE_MISSES: IntCounter = register_int_counter!(
        "cache_misses_total",
        "Total number of cache misses"
    ).unwrap();
}

/// Record API key validation metrics
pub fn record_validation_metrics(success: bool, error_type: Option<&str>, duration: std::time::Duration) {
    API_KEY_VALIDATION_TOTAL.inc();
    
    if success {
        API_KEY_VALIDATION_SUCCESS.inc();
    } else if let Some(reason) = error_type {
        API_KEY_VALIDATION_FAILED.with_label_values(&[reason]).inc();
    } else {
        API_KEY_VALIDATION_FAILED.with_label_values(&["unknown"]).inc();
    }
    
    API_KEY_VALIDATION_DURATION.observe(duration.as_secs_f64());
}

/// Record circuit breaker state change
pub fn record_circuit_breaker_state(state: &str) {
    CIRCUIT_BREAKER_STATE.with_label_values(&[state]).inc();
}

/// Record circuit breaker rejection
pub fn record_circuit_breaker_rejection() {
    CIRCUIT_BREAKER_REJECTED.inc();
}

/// Record cache hit
pub fn record_cache_hit() {
    CACHE_HITS.inc();
}

/// Record cache miss
pub fn record_cache_miss() {
    CACHE_MISSES.inc();
}

/// Collect all metrics for Prometheus
pub fn gather_metrics() -> Vec<u8> {
    let mut buffer = vec![];
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    buffer
}