use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::contracts::adapt::SessionUser;
use crate::tools::CoreError;
use crate::tools::request::CoreRequest;
use crate::tools::response::CoreResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResponse {
    pub user: Option<SessionUserResponse>,
    pub expires: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionUserResponse {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub image: Option<String>,
}

/// Get the current session
///
/// This endpoint validates the session token from cookies and returns the current user
pub async fn session(
    request: &CoreRequest<SessionRequest>,
) -> Result<CoreResponse<SessionResponse>, CoreError> {
    // Extract the session token from cookies
    let session_token = request.extract_session_token().map_err(|_| {
        CoreError::new()
            .with_message("No session token found")
            .with_status(StatusCode::UNAUTHORIZED.into())
    })?;

    // Get the adaptor
    let adaptor = request.extract_adaptor()?;

    // Get the session and user
    let session_user = adaptor
        .get_session_and_user(session_token)
        .await
        .ok_or_else(|| {
            CoreError::new()
                .with_message("Invalid or expired session")
                .with_status(StatusCode::UNAUTHORIZED.into())
        })?;

    let SessionUser { session, user } = session_user;

    // Return the user information
    let response = SessionResponse {
        user: Some(SessionUserResponse {
            id: user.id.unwrap_or_default(),
            name: user.username,
            email: user.email,
            image: user.image,
        }),
        expires: Some(
            chrono::DateTime::from_timestamp(i64::try_from(session.expires_in).unwrap_or(0), 0)
                .unwrap_or_default()
                .to_rfc3339(),
        ),
    };

    Ok(CoreResponse::<SessionResponse>::new().with_payload(&response))
}
