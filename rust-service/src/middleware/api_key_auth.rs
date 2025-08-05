use std::sync::Arc;
use std::collections::HashSet;
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    extract::State,
};
use thiserror::Error;
use serde::{Deserialize, Serialize};
use crate::clients::web_api::{WebApiClient, WebApiError};
use log::{info, warn, error};

#[derive(Debug, Error)]
pub enum ApiKeyAuthError {
    #[error("API key is missing")]
    MissingApiKey,
    
    #[error("Invalid API key")]
    InvalidApiKey,
    
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

impl From<WebApiError> for ApiKeyAuthError {
    fn from(err: WebApiError) -> Self {
        match err {
            WebApiError::ValidationError(msg) => {
                warn!("API key validation failed: {}", msg);
                ApiKeyAuthError::InvalidApiKey
            },
            WebApiError::RequestError(e) => {
                let status = e.status().map(|s| s.to_string()).unwrap_or_else(|| "unknown".to_string());
                let error_msg = format!("Request failed: {} (status: {})", e, status);
                error!("Service unavailable: {}", error_msg);
                ApiKeyAuthError::ServiceUnavailable(error_msg)
            },
            WebApiError::ServiceUnavailable(msg) => {
                error!("Service unavailable: {}", msg);
                ApiKeyAuthError::ServiceUnavailable(msg)
            },
            WebApiError::ResilientClientError(e) => {
                error!("Resilient client error: {}", e);
                ApiKeyAuthError::ServiceUnavailable(format!("Resilient client error: {}", e))
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ApiKeyAuthState {
    pub web_api_client: Arc<WebApiClient>,
    pub unlimited_api_keys: HashSet<String>,
}

pub async fn api_key_auth(
    State(state): State<Arc<ApiKeyAuthState>>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    // Extract API key from headers
    let api_key = req.headers()
        .get("x-api-key")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| {
            warn!("API key is missing from request");
            (StatusCode::UNAUTHORIZED, "API key is required".to_string())
        })?;

    info!("Validating API key (first 8 chars: {}...)", &api_key[..api_key.len().min(8)]);

    // Check if the provided key is an unlimited key
    let is_unlimited_key = state.unlimited_api_keys.contains(&api_key);
    
    // If it's not an unlimited key, validate it with the web API
    let validation = if is_unlimited_key {
        info!("Unlimited API key detected, bypassing regular validation");
        // Create a validation response with default values for unlimited keys
        crate::clients::web_api::ApiKeyValidationResponse {
            valid: true,
            user_id: None,
            email: Some("unlimited@example.com".to_string()),
            role: Some("unlimited".to_string()),
        }
    } else {
        // Validate API key with web-api
        state.web_api_client
            .validate_api_key(&api_key)
            .await
            .map_err(|e| {
                let error_msg = e.to_string();
                match e {
                    WebApiError::ValidationError(_) => {
                        warn!("Invalid API key provided");
                        (StatusCode::UNAUTHORIZED, "Invalid API key".to_string())
                    },
                    WebApiError::RequestError(_) | WebApiError::ServiceUnavailable(_) | WebApiError::ResilientClientError(_) => {
                        error!("Service unavailable during API key validation: {}", error_msg);
                        (StatusCode::SERVICE_UNAVAILABLE, "Service unavailable".to_string())
                    }
                }
            })?
    };

    if !validation.valid {
        warn!("API key validation returned invalid status");
        return Err((StatusCode::UNAUTHORIZED, "Invalid API key".to_string()));
    }

    info!(
        "API key validated for user: {:?} (role: {:?})", 
        validation.email.as_deref().unwrap_or("unknown"), 
        validation.role.as_deref().unwrap_or("unknown")
    );

    // Attach user information to the request extensions
    let user = AuthenticatedUser {
        user_id: validation.user_id,
        email: validation.email,
        role: validation.role,
    };
    
    req.extensions_mut().insert(user);
    
    Ok(next.run(req).await)
}
