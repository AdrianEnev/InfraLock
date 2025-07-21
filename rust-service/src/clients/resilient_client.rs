use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use tokio::sync::Mutex;
use moka::sync::Cache;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;
use tracing::{error, warn, info};

#[derive(Error, Debug)]
pub enum ResilientClientError {
    #[error("Request failed after maximum retries")]
    MaxRetriesExceeded,
    
    #[error("Service unavailable")]
    ServiceUnavailable,
    
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

#[derive(Clone, Debug)]
pub struct CircuitBreaker {
    failure_threshold: usize,
    reset_timeout: Duration,
    failures: Arc<Mutex<VecDeque<Instant>>>,
    state: Arc<Mutex<CircuitState>>,
}

#[derive(Debug, Clone, PartialEq)]
enum CircuitState {
    Closed,
    Open(Instant),
    HalfOpen,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, reset_timeout: Duration) -> Self {
        Self {
            failure_threshold,
            reset_timeout,
            failures: Arc::new(Mutex::new(VecDeque::with_capacity(failure_threshold))),
            state: Arc::new(Mutex::new(CircuitState::Closed)),
        }
    }

    pub async fn is_available(&self) -> bool {
        let state = self.state.lock().await;
        match *state {
            CircuitState::Closed => true,
            CircuitState::Open(until) => {
                if Instant::now() >= until {
                    // Transition to half-open
                    drop(state);
                    let mut state = self.state.lock().await;
                    *state = CircuitState::HalfOpen;
                    true
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => true,
        }
    }

    pub async fn record_success(&self) {
        let mut state = self.state.lock().await;
        if *state == CircuitState::HalfOpen {
            // Transition back to closed state
            *state = CircuitState::Closed;
            let mut failures = self.failures.lock().await;
            failures.clear();
        }
    }

    pub async fn record_failure(&self) {
        let mut failures = self.failures.lock().await;
        let now = Instant::now();
        
        // Remove old failures
        failures.retain(|&time| now.duration_since(time) < self.reset_timeout);
        
        failures.push_back(now);
        
        if failures.len() >= self.failure_threshold {
            let mut state = self.state.lock().await;
            *state = CircuitState::Open(now + self.reset_timeout);
            error!("Circuit breaker opened due to too many failures");
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResilientClient {
    client: Client,
    circuit_breaker: CircuitBreaker,
    cache: Arc<Cache<String, (String, Instant)>>,
    base_url: String,
}

impl ResilientClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            circuit_breaker: CircuitBreaker::new(5, Duration::from_secs(30)),
            cache: Arc::new(Cache::new(10_000)), // Cache up to 10,000 responses
            base_url,
        }
    }

    pub async fn get_cached<T: DeserializeOwned + Send + 'static>(
        &self,
        path: &str,
        cache_ttl: Duration,
    ) -> Result<T, ResilientClientError> {
        let cache_key = format!("GET:{}", path);
        
        // Check cache first
        if let Some((cached_response, cached_at)) = self.cache.get(&cache_key) {
            if cached_at.elapsed() < cache_ttl {
                info!("Cache hit for {}", path);
                return serde_json::from_str(&cached_response)
                    .map_err(|e| ResilientClientError::InvalidResponse(e.to_string()));
            }
        }
        
        // Not in cache or expired, make the request
        let response = self.get_with_retry(path).await?;
        
        // Parse and cache the response
        let response_text = response.text().await?;
        
        // Cache the response
        self.cache.insert(cache_key, (response_text.clone(), Instant::now()));
        
        // Parse and return the response
        serde_json::from_str(&response_text)
            .map_err(|e| ResilientClientError::InvalidResponse(e.to_string()))
    }
    
    pub async fn post_json<T: Serialize, U: DeserializeOwned + Send + 'static>(
        &self,
        path: &str,
        body: &T,
        cache_ttl: Duration,
    ) -> Result<U, ResilientClientError> {
        self.post_json_with_headers(path, body, cache_ttl, &[]).await
    }
    
    pub async fn post_json_with_headers<T: Serialize, U: DeserializeOwned + Send + 'static>(
        &self,
        path: &str,
        body: &T,
        cache_ttl: Duration,
        headers: &[(String, String)],
    ) -> Result<U, ResilientClientError> {
        let cache_key = format!("POST:{}:{:?}", path, serde_json::to_string(body));
        
        // Check cache first
        if let Some((cached_response, cached_at)) = self.cache.get(&cache_key) {
            if cached_at.elapsed() < cache_ttl {
                info!("Cache hit for POST {}", path);
                return serde_json::from_str(&cached_response)
                    .map_err(|e| ResilientClientError::InvalidResponse(e.to_string()));
            }
        }
        
        // Not in cache or expired, make the request
        let response = self.post_with_retry(path, body, headers).await?;
        
        // Parse and cache the response
        let response_text = response.text().await?;
        
        // Cache the response
        self.cache.insert(cache_key, (response_text.clone(), Instant::now()));
        
        // Parse and return the response
        serde_json::from_str(&response_text)
            .map_err(|e| ResilientClientError::InvalidResponse(e.to_string()))
    }
    
    async fn post_with_retry<T: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &T,
        headers: &[(String, String)],
    ) -> Result<Response, ResilientClientError> {
        const MAX_RETRIES: usize = 3;
        const INITIAL_RETRY_DELAY: Duration = Duration::from_millis(100);
        
        let mut last_error = None;
        
        for attempt in 0..=MAX_RETRIES {
            // Check circuit breaker
            if !self.circuit_breaker.is_available().await {
                warn!(
                    "Service unavailable - Circuit breaker is open. Path: {}, Attempt: {}/{}",
                    path,
                    attempt,
                    MAX_RETRIES
                );
                return Err(ResilientClientError::ServiceUnavailable);
            }
            
            // Exponential backoff
            if attempt > 0 {
                let delay = INITIAL_RETRY_DELAY * 2u32.pow(attempt as u32 - 1);
                info!("Retry attempt {} with delay {:?}", attempt, delay);
                tokio::time::sleep(delay).await;
            }
            
            let url = format!("{}{}", self.base_url, path);
            info!("Making request to: {}", url);
            
            let mut request = self.client.post(&url).json(body);
            
            // Add headers to the request
            for (key, value) in headers {
                request = request.header(key, value);
                info!("Added header: {}: {}", key, value);
            }
            
            match request.send().await {
                Ok(response) => {
                    let status = response.status();
                    info!("Received response: {}", status);
                    
                    if status.is_success() {
                        info!("Request successful - Status: {} - Path: {}", status, path);
                        self.circuit_breaker.record_success().await;
                        return Ok(response);
                    } else if status.is_server_error() {
                        // Record failure for server errors
                        warn!(
                            "Server error - Status: {} - Path: {} - Attempt: {}/{}",
                            status, path, attempt, MAX_RETRIES
                        );
                        self.circuit_breaker.record_failure().await;
                        last_error = Some(ResilientClientError::RequestError(
                            response.error_for_status().unwrap_err()
                        ));
                    } else {
                        // For client errors, don't retry
                        warn!(
                            "Client error - Status: {} - Path: {} - Not retrying",
                            status, path
                        );
                        return Err(ResilientClientError::RequestError(
                            response.error_for_status().unwrap_err()
                        ));
                    }
                }
                Err(e) => {
                    warn!(
                        "Request failed - Path: {} - Error: {} - Attempt: {}/{}",
                        path, e, attempt, MAX_RETRIES
                    );
                    self.circuit_breaker.record_failure().await;
                    last_error = Some(ResilientClientError::RequestError(e));
                }
            }
        }
        
        warn!("All retry attempts failed");
        Err(last_error.unwrap_or(ResilientClientError::MaxRetriesExceeded))
    }
    
    async fn get_with_retry(
        &self,
        path: &str,
    ) -> Result<Response, ResilientClientError> {
        self.get_with_retry_headers(path, &[]).await
    }
    
    async fn get_with_retry_headers(
        &self,
        path: &str,
        headers: &[(String, String)],
    ) -> Result<Response, ResilientClientError> {
        const MAX_RETRIES: usize = 3;
        const INITIAL_RETRY_DELAY: Duration = Duration::from_millis(100);
        
        let mut last_error = None;
        
        for attempt in 0..=MAX_RETRIES {
            // Check circuit breaker
            if !self.circuit_breaker.is_available().await {
                warn!(
                    "Service unavailable - Circuit breaker is open. Path: {}, Attempt: {}/{}",
                    path,
                    attempt,
                    MAX_RETRIES
                );
                return Err(ResilientClientError::ServiceUnavailable);
            }
            
            // Exponential backoff
            if attempt > 0 {
                let delay = INITIAL_RETRY_DELAY * 2u32.pow(attempt as u32 - 1);
                info!("Retry attempt {} with delay {:?}", attempt, delay);
                tokio::time::sleep(delay).await;
            }
            
            let url = format!("{}{}", self.base_url, path);
            
            let mut request = self.client.get(&url);
            
            // Add headers to the request
            for (key, value) in headers {
                request = request.header(key, value);
            }
            
            match request.send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        info!("Request successful - Status: {} - Path: {}", response.status(), path);
                        self.circuit_breaker.record_success().await;
                        return Ok(response);
                    } else if response.status().is_server_error() {
                        // Record failure for server errors
                        warn!(
                            "Server error - Status: {} - Path: {} - Attempt: {}/{}",
                            response.status(), path, attempt, MAX_RETRIES
                        );
                        self.circuit_breaker.record_failure().await;
                        last_error = Some(ResilientClientError::RequestError(
                            response.error_for_status().unwrap_err()
                        ));
                    } else {
                        // For client errors, don't retry
                        warn!(
                            "Client error - Status: {} - Path: {} - Not retrying",
                            response.status(), path
                        );
                        return Err(ResilientClientError::RequestError(
                            response.error_for_status().unwrap_err()
                        ));
                    }
                }
                Err(e) => {
                    warn!(
                        "Request failed - Path: {} - Error: {} - Attempt: {}/{}",
                        path, e, attempt, MAX_RETRIES
                    );
                    self.circuit_breaker.record_failure().await;
                    last_error = Some(ResilientClientError::RequestError(e));
                }
            }
        }
        
        warn!("All retry attempts failed");
        Err(last_error.unwrap_or(ResilientClientError::MaxRetriesExceeded))
    }

    pub async fn reset_circuit_breaker(&self) {
        let mut state = self.circuit_breaker.state.lock().await;
        let mut failures = self.circuit_breaker.failures.lock().await;
        
        info!(
            "Resetting circuit breaker. Previous state: {:?}, Failure count: {}",
            *state,
            failures.len()
        );
        
        *state = CircuitState::Closed;
        failures.clear();
    }
}
