use std::sync::Arc;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::time::Duration;
use crate::clients::resilient_client::{ResilientClient, ResilientClientError};
use std::time::Instant;
use serde_json::json;
use log::{info, warn, error};

#[derive(Debug, Error)]
pub enum WebApiError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("API key validation failed: {0}")]
    ValidationError(String),
    
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    
    #[error("Resilient client error: {0}")]
    ResilientClientError(#[from] ResilientClientError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyValidationResponse {
    pub valid: bool,
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CachedApiKey {
    pub valid_until: Instant,
    pub user_id: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Clone)]
pub struct WebApiClientConfig {
    pub base_url: String,
    pub service_token: String,
    pub cache_ttl: Duration,
}

impl Default for WebApiClientConfig {
    fn default() -> Self {
        Self {
            base_url: std::env::var("WEB_API_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            service_token: std::env::var("INTERNAL_SERVICE_TOKEN")
                .unwrap_or_else(|_| "default-service-token".to_string()),
            cache_ttl: Duration::from_secs(300), // 5 minutes default cache TTL
        }
    }
}

#[derive(Debug, Clone)]
pub struct WebApiClient {
    client: ResilientClient,
    config: WebApiClientConfig,
    cache: Arc<tokio::sync::RwLock<lru::LruCache<String, CachedApiKey>>>,
}

impl WebApiClient {
    pub fn new(config: WebApiClientConfig) -> Self {
        let client = ResilientClient::new(config.base_url.clone());
        
        let cache = Arc::new(tokio::sync::RwLock::new(
            lru::LruCache::new(std::num::NonZeroUsize::new(1000).unwrap())
        ));
        
        Self {
            client,
            config,
            cache,
        }
    }
    
    pub async fn validate_api_key(&self, api_key: &str) -> Result<ApiKeyValidationResponse, WebApiError> {
        // Check cache first
        if let Some(cached) = self.check_cache(api_key).await {
            info!("API key found in cache for user: {:?}", cached.email);
            return Ok(cached);
        }
        
        info!("Validating API key (not found in cache)");
        
        let path = "/internal/validate-key";
        
        let response_result = self.client
            .post_json_with_headers::<serde_json::Value, ApiKeyValidationResponse>(
                path,
                &serde_json::json!({}),
                self.config.cache_ttl,
                &[
                    ("Authorization".to_string(), format!("Bearer {}", self.config.service_token)),
                    ("x-api-key".to_string(), api_key.to_string())
                ]
            )
            .await
            .map_err(|e| {
                warn!("API key validation request failed: {}", e);
                e
            });
            
        match response_result {
            Ok(mut response) => {
                if !response.valid {
                    warn!("API key validation failed: Invalid API key");
                    return Err(WebApiError::ValidationError("Invalid API key".to_string()));
                }
                
                // Check for required fields
                if response.user_id.is_none() || response.email.is_none() || response.role.is_none() {
                    let error_msg = format!("Missing required fields in API response. Fields present: user_id={:?}, email={:?}, role={:?}",
                        response.user_id.is_some(),
                        response.email.is_some(),
                        response.role.is_some()
                    );
                    error!("Invalid API response: {}", error_msg);
                    return Err(WebApiError::ServiceUnavailable(error_msg));
                }
                
                info!("API key validated successfully for user: {:?}", response.email);
                
                // Take ownership of the fields we need
                let user_id = response.user_id.take().expect("user_id should be present");
                let email = response.email.take().expect("email should be present");
                let role = response.role.take().expect("role should be present");
                
                // Cache the result if valid
                self.cache_api_key(
                    api_key.to_string(),
                    user_id,
                    email,
                    role,
                ).await;
                
                Ok(response)
            }
            Err(ResilientClientError::RequestError(e)) => {
                if e.status() == Some(reqwest::StatusCode::UNAUTHORIZED) {
                    warn!("API key validation failed: Unauthorized (401)");
                    Err(WebApiError::ValidationError("Invalid API key".to_string()))
                } else {
                    let status = e.status().map(|s| s.as_u16()).unwrap_or(0);
                    let error_msg = format!("HTTP error {}: {}", status, e);
                    error!("Service unavailable: {}", error_msg);
                    Err(WebApiError::ServiceUnavailable(error_msg))
                }
            }
            Err(e) => {
                let error_msg = e.to_string();
                error!("Service unavailable: {}", error_msg);
                Err(WebApiError::ServiceUnavailable(error_msg))
            }
        }
    }
    
    async fn check_cache(&self, api_key: &str) -> Option<ApiKeyValidationResponse> {
        let mut cache = self.cache.write().await;
        
        if let Some(cached) = cache.get(api_key) {
            if cached.valid_until > Instant::now() {
                return Some(ApiKeyValidationResponse {
                    valid: true,
                    user_id: Some(cached.user_id.clone()),
                    email: Some(cached.email.clone()),
                    role: Some(cached.role.clone()),
                });
            } else {
                // Remove expired entry
                cache.pop(api_key);
            }
        }
        
        None
    }
    
    async fn cache_api_key(&self, api_key: String, user_id: String, email: String, role: String) {
        let mut cache = self.cache.write().await;
        
        let cached = CachedApiKey {
            valid_until: Instant::now() + self.config.cache_ttl,
            user_id,
            email,
            role,
        };
        
        cache.put(api_key, cached);
    }
    
    pub async fn reset_circuit_breaker(&self) {
        // This is a test method to reset the circuit breaker
        // In a real application, you'd want to handle this more carefully
        self.client.reset_circuit_breaker().await;
    }
}
