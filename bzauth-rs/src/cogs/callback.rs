use http::StatusCode;
use oauth2::{CsrfToken, PkceCodeChallenge};

use super::{CoreError, request::CoreRequest, response::CoreResponse};
use crate::{
    cogs,
    contracts::provide::{ProvideOAuth2, ProviderType},
};

// Handle the callback
pub async fn callback<Payload>(request: CoreRequest<Payload>) -> Result<CoreResponse, CoreError> {
    let provider = cogs::extract_provider(&request)?;

    let provider_type = provider.provider_type();
    if provider_type != ProviderType::OAuth {
        return Err(CoreError::new()
            .with_message("Only OAuth2 providers are supported".to_string())
            .with_status(StatusCode::BAD_REQUEST.into()));
    }

    let oauth2_provider = provider.as_any().downcast_ref::<Box<dyn ProvideOAuth2>>();
    if oauth2_provider.is_none() {
        return Err(CoreError::new()
            .with_message("Failed to downcast to OAuth2 provider".to_string())
            .with_status(StatusCode::BAD_REQUEST.into()));
    }

    let oauth2_provider = oauth2_provider.unwrap();
    let client = cogs::generate_client_from_auth(oauth2_provider)?;

    // Todo: Generate our own CsrfToken
    let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_challenge)
        .url();

    // Redirect the user to the authorization URL
    Ok(CoreResponse::redirect(auth_url.to_string()))
}
