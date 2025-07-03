use serde::{Deserialize, Serialize};

use super::response::CoreResponse;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoreError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CoreError {}

impl CoreError {
    /// Creates a new `CoreError` with a default status of 500 and a generic message.
    pub fn new() -> Self {
        CoreError {
            status: http::StatusCode::INTERNAL_SERVER_ERROR.into(),
            message: "Unknown error".to_string(),
        }
    }

    pub fn with_status(self, status: u16) -> Self {
        CoreError {
            status,
            message: self.message,
        }
    }
    pub fn with_message(self, message: String) -> Self {
        CoreError {
            status: self.status,
            message,
        }
    }
}

impl From<CoreError> for CoreResponse<String> {
    fn from(error: CoreError) -> Self {
        CoreResponse::<String>::new()
            .with_status(
                http::StatusCode::from_u16(error.status).unwrap_or(http::StatusCode::BAD_REQUEST),
            )
            .with_payload(error.message)
    }
}

impl Default for CoreError {
    fn default() -> Self {
        CoreError::new()
    }
}

/// Converts a CoreError into a Result type with the same error type.
impl<T> From<CoreError> for Result<T, CoreError> {
    fn from(err: CoreError) -> Self {
        Err(err)
    }
}

/// Converts a ParseError from the `oauth2` crate into a `CoreError`.
impl From<oauth2::url::ParseError> for CoreError {
    fn from(err: oauth2::url::ParseError) -> Self {
        CoreError::new().with_message(err.to_string())
    }
}
