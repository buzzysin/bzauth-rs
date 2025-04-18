use serde::{Deserialize, Serialize};

use super::{Cookies, response::CoreResponse};

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

impl From<CoreError> for CoreResponse {
    fn from(error: CoreError) -> Self {
        CoreResponse {
            status: http::StatusCode::from_u16(error.status)
                .unwrap_or(http::StatusCode::BAD_REQUEST),
            body: Some(error.message),
            headers: Default::default(),
            cookies: Cookies::new(),
        }
    }
}
