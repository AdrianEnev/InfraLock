pub mod validation;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maxminddb::MaxMindDbError;
use std::fmt::{self, Display};
use crate::errors::validation::IpValidationError;
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum AppError {
    ValidationError(IpValidationError),
    DatabaseError(SqlxError),
    MaxMindDbError(MaxMindDbError),
    ConfigError(config::ConfigError),
    AddrParseError(std::net::AddrParseError),
    IoError(std::io::Error),
    NotFound(String),
    InternalServerError,
}

impl std::error::Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::MaxMindDbError(e) => write!(f, "MaxMind DB error: {}", e),
            AppError::ConfigError(e) => write!(f, "Configuration error: {}", e),
            AppError::AddrParseError(e) => write!(f, "Address parse error: {}", e),
            AppError::IoError(e) => write!(f, "I/O error: {}", e),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::ValidationError(e) => write!(f, "Validation error: {}", e),
            AppError::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::MaxMindDbError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::ConfigError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::AddrParseError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::IoError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::ValidationError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };

        let body = serde_json::json!({ "error": error_message });
        (status, axum::Json(body)).into_response()
    }
}

// Implement From traits for easy error conversion
impl From<MaxMindDbError> for AppError {
    fn from(err: MaxMindDbError) -> Self {
        AppError::MaxMindDbError(err)
    }
}

impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        AppError::DatabaseError(err)
    }
}

impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::ConfigError(err)
    }
}

impl From<std::net::AddrParseError> for AppError {
    fn from(err: std::net::AddrParseError) -> Self {
        AppError::AddrParseError(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
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

impl From<IpValidationError> for AppError {
    fn from(err: IpValidationError) -> Self {
        AppError::ValidationError(err)
    }
}