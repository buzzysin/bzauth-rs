use http::StatusCode;

use crate::{
    cogs::{self, CoreError, request::CoreRequest, response::CoreResponse},
    contracts::provide::ProviderType,
};

// Handle the callback
pub async fn callback<Payload>(request: CoreRequest<Payload>) -> Result<CoreResponse, CoreError> {
    let provider = cogs::extract_provider(&request)?;
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

async fn callback_oauth2<Payload>(
    request: CoreRequest<Payload>,
) -> Result<CoreResponse, CoreError> {
    // Handle OAuth2 callback
    let provider = cogs::extract_provider(&request)?;
    let _oauth2_provider = provider
        .as_ref()
        .as_oauth2()
        .ok_or_else(|| CoreError::new().with_message("Provider is not OAuth2".to_string()))?;

    // Handle potential callback errors from the provider
    if let Some(error) = request.query().get("error") {
        return Err(CoreError::new()
            .with_message(format!("OAuth2 error: {}", error))
            .with_status(StatusCode::BAD_REQUEST.into()));
    }

    // Extract the authorization code from the request
    let (_code, _state) = cogs::extract_auth_code_and_state(&request)?;

    todo!()
}

async fn callback_email<Payload>(request: CoreRequest<Payload>) -> Result<CoreResponse, CoreError> {
    // Handle email provider callback
    let provider = cogs::extract_provider(&request)?;
    let provider_type = provider.provider_type();

    // So far unsupported
    Err(CoreError::new()
        .with_message(format!("Unsupported provider type: {}", provider_type))
        .with_status(StatusCode::BAD_REQUEST.into()))
}

async fn callback_credentials<Payload>(
    request: CoreRequest<Payload>,
) -> Result<CoreResponse, CoreError> {
    // Handle credentials provider callback
    let provider = cogs::extract_provider(&request)?;
    let provider_type = provider.provider_type();

    // So far unsupported
    Err(CoreError::new()
        .with_message(format!("Unsupported provider type: {}", provider_type))
        .with_status(StatusCode::BAD_REQUEST.into()))
}

async fn callback_oidc<Payload>(request: CoreRequest<Payload>) -> Result<CoreResponse, CoreError> {
    // Handle OIDC provider callback
    let provider = cogs::extract_provider(&request)?;
    let provider_type = provider.provider_type();

    // So far unsupported
    Err(CoreError::new()
        .with_message(format!("Unsupported provider type: {}", provider_type))
        .with_status(StatusCode::BAD_REQUEST.into()))
}
