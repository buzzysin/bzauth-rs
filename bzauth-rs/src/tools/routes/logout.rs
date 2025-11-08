use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::tools::CoreError;
use crate::tools::request::CoreRequest;
use crate::tools::response::CoreResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutResponse {
    pub success: bool,
}

/// Logout the current user
///
/// This endpoint invalidates the session and clears the session cookie
pub async fn logout(
    request: &CoreRequest<LogoutRequest>,
) -> Result<CoreResponse<LogoutResponse>, CoreError> {
    // Extract the session token from cookies
    let session_token = request.extract_session_token().map_err(|_| {
        CoreError::new()
            .with_message("No session token found")
            .with_status(StatusCode::UNAUTHORIZED.into())
    })?;

    // Get the adaptor
    let adaptor = request.extract_adaptor()?;

    // Delete the session
    adaptor.delete_session(session_token).await;

    // Return success and clear the session cookie
    let response_body = LogoutResponse { success: true };
    let response = CoreResponse::<LogoutResponse>::new()
        .with_payload(&response_body)
        .with_cookie("session".to_string(), String::new());

    Ok(response)
}
