use oauth2::{CsrfToken, PkceCodeChallenge};

use crate::{
    cogs::{self, CoreError, request::CoreRequest, response::CoreResponse},
    contracts::provide::ProviderType,
};

impl From<oauth2::url::ParseError> for CoreError {
    fn from(err: oauth2::url::ParseError) -> Self {
        CoreError::new().with_message(err.to_string())
    }
}

impl<T> From<CoreError> for Result<T, CoreError> {
    fn from(err: CoreError) -> Self {
        Err(err)
    }
}

pub async fn authorise<Payload>(request: CoreRequest<Payload>) -> Result<CoreResponse, CoreError> {
    // Extract the provider from the request
    let provider = cogs::extract_provider(&request)?;

    // Handle the callback
    let provider_type = provider.provider_type();

    match provider_type {
        ProviderType::OAuth => self::authorise_oauth2(request).await,
        ProviderType::Email => self::authorise_email(request).await,
        ProviderType::Credentials => self::authorise_credentials(request).await,
        ProviderType::OIDC => self::authorise_oidc(request).await,
    }
}

async fn authorise_oauth2<Payload>(
    request: CoreRequest<Payload>,
) -> Result<CoreResponse, CoreError> {
    // Handle OAuth2 callback
    let provider = cogs::extract_provider(&request)?;
    let oauth2_provider = provider
        .as_ref()
        .as_oauth2()
        .ok_or_else(|| CoreError::new().with_message("Provider is not OAuth2".to_string()))?;

    let client = cogs::generate_client_from_auth(oauth2_provider)?;

    let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (authorisation_url, _csrf_token) = client
         // todo: manual csrf token
         .authorize_url(CsrfToken::new_random)
         // todo: add scopes
         .set_pkce_challenge(pkce_challenge)
         .url();

    // Generate a state and store it in the session

    // Redirect to the authorization URL
    Ok(CoreResponse::redirect(authorisation_url.to_string()))
}

async fn authorise_email<Payload>(
    request: CoreRequest<Payload>,
) -> Result<CoreResponse, CoreError> {
    // Handle email provider authorisation
    let provider = cogs::extract_provider(&request)?;
    let provider_type = provider.provider_type();

    // So far unsupported
    CoreError::new()
        .with_message(format!("Unsupported provider type: {}", provider_type))
        .with_status(http::StatusCode::BAD_REQUEST.into())
        .into()
}

async fn authorise_credentials<Payload>(
    request: CoreRequest<Payload>,
) -> Result<CoreResponse, CoreError> {
    // Handle credentials provider authorisation
    let provider = cogs::extract_provider(&request)?;
    let provider_type = provider.provider_type();

    // So far unsupported
    CoreError::new()
        .with_message(format!("Unsupported provider type: {}", provider_type))
        .with_status(http::StatusCode::BAD_REQUEST.into())
        .into()
}

async fn authorise_oidc<Payload>(request: CoreRequest<Payload>) -> Result<CoreResponse, CoreError> {
    // Handle OIDC provider authorisation
    let provider = cogs::extract_provider(&request)?;
    let provider_type = provider.provider_type();

    // So far unsupported
    CoreError::new()
        .with_message(format!("Unsupported provider type: {}", provider_type))
        .with_status(http::StatusCode::BAD_REQUEST.into())
        .into()
}
