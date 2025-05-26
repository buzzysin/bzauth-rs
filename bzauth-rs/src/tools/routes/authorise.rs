use oauth2::CsrfToken;

use crate::tools::{
    cookie::Cookies,
    generators,
    request_extractors::{COOKIE_CSRF_TOKEN, COOKIE_STATE},
};
use crate::{
    contracts::provide::ProviderType,
    tools::{CoreError, request::CoreRequest, response::CoreResponse},
};

pub async fn authorise(request: CoreRequest) -> Result<CoreResponse, CoreError> {
    // Extract the provider from the request
    let provider = request.extract_provider()?;

    // Handle the callback
    let provider_type = provider.provider_type();

    // Dispatch to the appropriate authorisation function based on the provider type
    match provider_type {
        ProviderType::OAuth => self::authorise_oauth2(request).await,
        ProviderType::Email => self::authorise_email(request).await,
        ProviderType::Credentials => self::authorise_credentials(request).await,
        ProviderType::OIDC => self::authorise_oidc(request).await,
    }
}

async fn authorise_oauth2(request: CoreRequest) -> Result<CoreResponse, CoreError> {
    // Stage the response with the request data
    let mut response = CoreResponse::from_request(&request);

    // Extract the provider from the request
    let provider = request.extract_provider()?;
    let oauth2_provider = provider
        .as_ref()
        .as_oauth2()
        .ok_or_else(|| CoreError::new().with_message("Provider is not OAuth2".to_string()))?;

    let client = generators::generate_client_from_auth(oauth2_provider)?;
    // let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let state = generators::generate_state();
    let (authorisation_url, csrf_token) = client
         // todo: manual csrf token
         .authorize_url(CsrfToken::new_random)
         // todo: add scopes
        // .add_scopes(&oauth2_provider.scopes())
        // .set_pkce_challenge(pkce_challenge)
         .url();

    {
        // Set the cookies in the response
        let mut cookies = Cookies::new();
        cookies.set(COOKIE_STATE, state.clone());
        cookies.set(COOKIE_CSRF_TOKEN, csrf_token.secret().to_string());
        // TODO: Set the PKCE verifier cookie if needed
        response = response.with_cookies(cookies);
    }

    tracing::debug!("[authorise] Authorisation URL: {}", authorisation_url);
    tracing::debug!(
        "[authorise] State: {:?}",
        response.cookies().get("state").map(|c| c.value)
    );
    tracing::debug!(
        "[authorise] CSRF Token: {:?}",
        response.cookies().get("csrf_token").map(|c| c.value)
    );

    // Redirect to the authorization URL
    Ok(response.with_redirect(authorisation_url.to_string()))
}

// ignore
async fn authorise_email(request: CoreRequest) -> Result<CoreResponse, CoreError> {
    // Handle email provider authorisation
    let provider = request.extract_provider()?;
    let provider_type = provider.provider_type();

    // So far unsupported
    CoreError::new()
        .with_message(format!("Unsupported provider type: {}", provider_type))
        .with_status(http::StatusCode::BAD_REQUEST.into())
        .into()
}

// ignore
async fn authorise_credentials(request: CoreRequest) -> Result<CoreResponse, CoreError> {
    // Handle credentials provider authorisation
    let provider = request.extract_provider()?;
    let provider_type = provider.provider_type();

    // So far unsupported
    CoreError::new()
        .with_message(format!("Unsupported provider type: {}", provider_type))
        .with_status(http::StatusCode::BAD_REQUEST.into())
        .into()
}

// ignore
async fn authorise_oidc(request: CoreRequest) -> Result<CoreResponse, CoreError> {
    // Handle OIDC provider authorisation
    let provider = request.extract_provider()?;
    let provider_type = provider.provider_type();

    // So far unsupported
    CoreError::new()
        .with_message(format!("Unsupported provider type: {}", provider_type))
        .with_status(http::StatusCode::BAD_REQUEST.into())
        .into()
}
