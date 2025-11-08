use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::contracts::adapt::ProviderAccountId;
use crate::contracts::token::Token;
use crate::tools::request::CoreRequest;
use crate::tools::response::CoreResponse;
use crate::tools::{CoreError, generators};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshRequest {
    pub provider_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshResponse {
    pub success: bool,
    pub access_token: Option<String>,
    pub expires_in: Option<u64>,
}

/// Refresh an access token using the stored refresh token
///
/// This endpoint attempts to refresh the access token for a provider using the stored refresh token
pub async fn refresh(
    request: &CoreRequest<RefreshRequest>,
) -> Result<CoreResponse<RefreshResponse>, CoreError> {
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

    let user = session_user.user;
    let user_id = user.id.ok_or_else(|| {
        CoreError::new()
            .with_message("User ID not found")
            .with_status(StatusCode::INTERNAL_SERVER_ERROR.into())
    })?;

    // Get the provider
    let provider = request.extract_provider()?;
    let oauth2_provider = provider
        .as_ref()
        .as_oauth2()
        .ok_or_else(|| CoreError::new().with_message("Provider is not OAuth2"))?;

    // Get the account for this provider
    let provider_id = oauth2_provider.id();
    let account = adaptor
        .get_account(ProviderAccountId {
            provider_id: provider_id.clone(),
            provider_account_id: user_id.clone(),
        })
        .await
        .ok_or_else(|| {
            CoreError::new()
                .with_message("No account found for this provider")
                .with_status(StatusCode::NOT_FOUND.into())
        })?;

    // Get the refresh token from the account
    let refresh_token = account
        .token
        .as_ref()
        .and_then(|t| t.refresh_token.clone())
        .ok_or_else(|| {
            CoreError::new()
                .with_message("No refresh token available")
                .with_status(StatusCode::BAD_REQUEST.into())
        })?;

    // Exchange the refresh token for a new access token
    let client = generators::generate_client_from_provider(oauth2_provider)?;
    let token_response = client
        .exchange_refresh_token(&oauth2::RefreshToken::new(refresh_token))
        .request_async(&generators::generate_http_client()?)
        .await
        .map_err(|e| {
            CoreError::new()
                .with_message(format!("Failed to refresh token: {e}"))
                .with_status(StatusCode::BAD_REQUEST.into())
        })?;

    // Update the account with the new token
    let new_token = Token::try_from(token_response.clone()).map_err(|e| {
        CoreError::new()
            .with_message(format!("Failed to convert token response: {e}"))
            .with_status(StatusCode::INTERNAL_SERVER_ERROR.into())
    })?;

    let updated_account = adaptor
        .link_account(crate::contracts::account::Account {
            id: account.id,
            user_id: Some(user_id),
            provider_id: Some(provider_id),
            provider_type: oauth2_provider.provider_type(),
            provider_account_id: account.provider_account_id,
            token: Some(new_token.clone()),
        })
        .await
        .ok_or_else(|| {
            CoreError::new()
                .with_message("Failed to update account")
                .with_status(StatusCode::INTERNAL_SERVER_ERROR.into())
        })?;

    tracing::debug!("[refresh] Updated account: {:?}", updated_account);

    // Return the new access token info
    let response = RefreshResponse {
        success: true,
        access_token: new_token.access_token,
        expires_in: new_token.expires_in,
    };

    Ok(CoreResponse::<RefreshResponse>::new().with_payload(&response))
}
