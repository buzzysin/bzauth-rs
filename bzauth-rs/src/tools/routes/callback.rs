use http::StatusCode;
use oauth2::{AuthorizationCode, TokenResponse};

use crate::{
    contracts::{profile::Profile, provide::ProviderType},
    tools::{CoreError, generators, request::CoreRequest, response::CoreResponse},
};

// Handle the callback
pub async fn callback(request: CoreRequest) -> Result<CoreResponse, CoreError> {
    let provider = request.extract_provider()?;
    let provider_type = provider.provider_type();

    // The user has just completed the OAuth2 flow and is being redirected back to the application
    // with an authorization code. We need to exchange this code for an access token.

    match provider_type {
        ProviderType::OAuth => callback_oauth2(request).await,
        ProviderType::Email => callback_email(request).await,
        ProviderType::Credentials => callback_credentials(request).await,
        ProviderType::OIDC => callback_oidc(request).await,
    }
}

async fn callback_oauth2(request: CoreRequest) -> Result<CoreResponse, CoreError> {
    // Handle potential callback errors from the provider
    if let Some(error) = request.query().get("error") {
        return Err(CoreError::new()
            .with_message(format!("OAuth2 error: {}", error))
            .with_status(StatusCode::BAD_REQUEST.into()));
    }

    // Handle OAuth2 code callback
    let provider = request.extract_provider()?;
    let oauth2_provider = provider
        .as_ref()
        .as_oauth2()
        .ok_or_else(|| CoreError::new().with_message("Provider is not OAuth2".to_string()))?;

    // Extract the authorization code from the request
    let code = AuthorizationCode::new(request.extract_code()?);
    tracing::debug!("[callback] Code: {}", code.secret());

    // Exchange the authorization code for an access token
    let client = generators::generate_client_from_auth(oauth2_provider)?;
    let token_response = client
        .exchange_code(code)
        .request_async(&generators::generate_http_client()?)
        .await
        .map_err(|e| {
            CoreError::new()
                .with_message(format!("Failed to exchange code: (error={})", e))
                .with_status(StatusCode::BAD_REQUEST.into())
        })?;

    tracing::debug!("[callback] Token: {:?}", token_response.access_token());

    // Create a response with the token information
    let response = CoreResponse::from_request(&request);

    // We have successfully exchanged the code for a token, now we need to do a profile request
    // to populate the profile
    let profile_endpoint = oauth2_provider.profile_endpoint();
    let profile_client = generators::generate_http_client()?;
    let profile_response = profile_client
        .get(profile_endpoint.url())
        .bearer_auth(token_response.access_token().secret())
        .send()
        .await
        .map_err(|e| {
            CoreError::new()
                .with_message(format!("Failed to fetch user info: (error={})", e))
                .with_status(StatusCode::BAD_REQUEST.into())
        })?
        .json::<Profile>()// TODO: Change to a proper user info type
        .await
        .map_err(|e| {
            CoreError::new()
                .with_message(format!("Failed to parse user info: (error={})", e))
                .with_status(StatusCode::BAD_REQUEST.into())
        })?;

    tracing::debug!("[callback] User Info: {:?}", profile_response);

    // Set the user info in the response payload
    /* response = response.with_payload(serde_json::json!(
        {
            "access_token": token_response.access_token().secret(),
            "refresh_token": token_response.refresh_token().map(|t| t.secret()),
            "expires_in": token_response.expires_in().map(|d| d.as_secs()),
            "user_info": userinfo_response
        }
    )); */

    Ok(response)
}

// ignore
async fn callback_email(request: CoreRequest) -> Result<CoreResponse, CoreError> {
    // Handle email provider callback
    let provider = request.extract_provider()?;
    let provider_type = provider.provider_type();

    // So far unsupported
    Err(CoreError::new()
        .with_message(format!("Unsupported provider type: {}", provider_type))
        .with_status(StatusCode::BAD_REQUEST.into()))
}

// ignore
async fn callback_credentials(request: CoreRequest) -> Result<CoreResponse, CoreError> {
    // Handle credentials provider callback
    let provider = request.extract_provider()?;
    let provider_type = provider.provider_type();

    // So far unsupported
    Err(CoreError::new()
        .with_message(format!("Unsupported provider type: {}", provider_type))
        .with_status(StatusCode::BAD_REQUEST.into()))
}

// ignore
async fn callback_oidc(request: CoreRequest) -> Result<CoreResponse, CoreError> {
    // Handle OIDC provider callback
    let provider = request.extract_provider()?;
    let provider_type = provider.provider_type();

    // So far unsupported
    Err(CoreError::new()
        .with_message(format!("Unsupported provider type: {}", provider_type))
        .with_status(StatusCode::BAD_REQUEST.into()))
}
