use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maxminddb::MaxMindDbError;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    MaxMindDb(MaxMindDbError),
    Config(config::ConfigError),
    AddrParse(std::net::AddrParseError),
    Io(std::io::Error),
    NotFound(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::MaxMindDb(e) => write!(f, "MaxMind DB error: {}", e),
            AppError::Config(e) => write!(f, "Configuration error: {}", e),
            AppError::AddrParse(e) => write!(f, "Address parse error: {}", e),
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::MaxMindDb(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Config(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::AddrParse(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::Io(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
        };

        let body = serde_json::json!({ "error": error_message });
        (status, axum::Json(body)).into_response()
    }
}

// Implement From traits for easy error conversion
impl From<MaxMindDbError> for AppError {
    fn from(err: MaxMindDbError) -> Self {
        AppError::MaxMindDb(err)
    }
}

impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::Config(err)
    }
}

impl From<std::net::AddrParseError> for AppError {
    fn from(err: std::net::AddrParseError) -> Self {
        AppError::AddrParse(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::NotFound(err)
    }
}

impl From<&str> for AppError {
    fn from(err: &str) -> Self {
        AppError::NotFound(err.to_string())
    }
}