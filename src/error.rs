use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    RedisError(redis::RedisError),
    ConfigError(envconfig::Error),
    HttpError(reqwest::Error),
    JsonError(serde_json::Error),
    UrlEncodedError(serde_urlencoded::ser::Error),
    IoError(std::io::Error),
    SpotifyError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(err) => write!(f, "Database error: {}", err),
            AppError::RedisError(err) => write!(f, "Redis error: {}", err),
            AppError::ConfigError(err) => write!(f, "Configuration error: {}", err),
            AppError::HttpError(err) => write!(f, "HTTP error: {}", err),
            AppError::JsonError(err) => write!(f, "JSON error: {}", err),
            AppError::UrlEncodedError(err) => write!(f, "URL encoding error: {}", err),
            AppError::IoError(err) => write!(f, "IO error: {}", err),
            AppError::SpotifyError(msg) => write!(f, "Spotify error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": self.to_string()
        }))
    }
}

// Conversion implementations
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        AppError::RedisError(err)
    }
}

impl From<envconfig::Error> for AppError {
    fn from(err: envconfig::Error) -> Self {
        AppError::ConfigError(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::HttpError(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::JsonError(err)
    }
}

impl From<serde_urlencoded::ser::Error> for AppError {
    fn from(err: serde_urlencoded::ser::Error) -> Self {
        AppError::UrlEncodedError(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}
