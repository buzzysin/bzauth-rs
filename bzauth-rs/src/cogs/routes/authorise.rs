use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, TokenUrl,
    basic::BasicClient,
};

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

    let auth_url = oauth2_provider.auth_endpoint().url();
    let client_id = oauth2_provider.client_id();
    let client_secret = oauth2_provider.client_secret();

    let client_id = ClientId::new(client_id.to_string());
    let client_secret = ClientSecret::new(client_secret.to_string());
    let auth_url = AuthUrl::new(auth_url.to_string())?;
    // TODO: The redirect URL should be configurable or be present in the request
    let redirect_url = RedirectUrl::new("http://localhost:3001/".to_string())?;
    let token_url = TokenUrl::new(oauth2_provider.token_endpoint().url())?;

    let client = BasicClient::new(client_id)
        .set_auth_uri(auth_url)
        .set_redirect_uri(redirect_url)
        .set_token_uri(token_url)
        .set_client_secret(client_secret);

    let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, _csrf_token) = client
         // todo: manual csrf token
         .authorize_url(CsrfToken::new_random)
         // todo: add scopes
         .set_pkce_challenge(pkce_challenge)
         .url();

    // Redirect to the authorization URL
    Ok(CoreResponse::redirect(auth_url.to_string()))
}

async fn authorise_email<Payload>(
    request: CoreRequest<Payload>,
) -> Result<CoreResponse, CoreError> {
    // Handle email provider authorisation
    let provider = cogs::extract_provider(&request)?;
    let provider_type = provider.provider_type();

    // Handle other providers
    CoreError::new()
        .with_message(format!(
            "Unsupported provider type: {}",
            provider_type.to_string()
        ))
        .with_status(http::StatusCode::BAD_REQUEST.into())
        .into()
}

async fn authorise_credentials<Payload>(
    request: CoreRequest<Payload>,
) -> Result<CoreResponse, CoreError> {
    // Handle credentials provider authorisation
    let provider = cogs::extract_provider(&request)?;
    let provider_type = provider.provider_type();

    // Handle other providers
    CoreError::new()
        .with_message(format!(
            "Unsupported provider type: {}",
            provider_type.to_string()
        ))
        .with_status(http::StatusCode::BAD_REQUEST.into())
        .into()
}

async fn authorise_oidc<Payload>(
    request: CoreRequest<Payload>,
) -> Result<CoreResponse, CoreError> {
    // Handle OIDC provider authorisation
    let provider = cogs::extract_provider(&request)?;
    let provider_type = provider.provider_type();

    // Handle other providers
    CoreError::new()
        .with_message(format!(
            "Unsupported provider type: {}",
            provider_type.to_string()
        ))
        .with_status(http::StatusCode::BAD_REQUEST.into())
        .into()
}