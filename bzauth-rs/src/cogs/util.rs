use std::sync::Arc;

use oauth2::{
    AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl, TokenUrl,
    basic::BasicClient,
};

use super::CoreError;
use crate::contracts::provide::{Provide, ProvideOAuth2};
use crate::{auth::Auth, cogs::request::CoreRequest};

pub enum UtilError {
    MissingAuth(String),
    MissingProviderId(String),
    MissingProvider(String),
}

impl From<UtilError> for CoreError {
    fn from(error: UtilError) -> Self {
        match error {
            UtilError::MissingAuth(msg) => CoreError::new().with_message(msg).with_status(500),
            UtilError::MissingProviderId(msg) => {
                CoreError::new().with_message(msg).with_status(500)
            }
            UtilError::MissingProvider(msg) => CoreError::new().with_message(msg).with_status(500),
        }
    }
}

pub fn extract_auth<Payload>(request: &CoreRequest<Payload>) -> Result<Arc<Auth>, UtilError> {
    let auth = request
        .auth
        .as_ref()
        .ok_or_else(|| UtilError::MissingAuth("Auth not found".to_string()))?
        .clone();

    Ok(auth)
}

pub fn extract_provider_id<Payload>(request: &CoreRequest<Payload>) -> Result<String, UtilError> {
    let path = request.path.clone();
    let provider_id = path
        .split('/')
        .nth(2)
        .ok_or_else(|| UtilError::MissingProviderId("Provider ID not found".to_string()))?
        .to_string();

    Ok(provider_id)
}

pub fn extract_provider<Payload>(
    request: &CoreRequest<Payload>,
) -> Result<Box<dyn Provide>, UtilError> {
    let provider_id = extract_provider_id(request)?;
    let auth = extract_auth(request)?;

    let provider = auth
        .options
        .providers
        .iter()
        .find(|p| p.id() == provider_id)
        .ok_or_else(|| UtilError::MissingProvider("Provider not found".to_string()))?
        .clone();

    Ok(provider)
}

pub fn extract_auth_code_and_state<Payload>(
    request: &CoreRequest<Payload>,
) -> Result<(String, String), UtilError> {
    let code = request
        .query()
        .get("code")
        .ok_or_else(|| UtilError::MissingAuth("Missing authorization code".to_string()))?
        .to_string();

    let state = request
        .query()
        .get("state")
        .ok_or_else(|| UtilError::MissingAuth("Missing state".to_string()))?
        .to_string();

    Ok((code, state))
}

type CatchAllClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

pub fn generate_client_from_auth(
    oauth2_provider: &dyn ProvideOAuth2,
) -> Result<CatchAllClient, UtilError> {
    let auth_url = oauth2_provider.auth_endpoint().url();
    let client_id = oauth2_provider.client_id();
    let client_secret = oauth2_provider.client_secret();
    let redirect_url = "http://localhost:3000/".to_string(); // This should be the redirect URL you set in your provider settings
    let token_url = oauth2_provider.token_endpoint().url();

    // Convert everything to oauth2 types
    let client_id = ClientId::new(client_id.to_string());
    let client_secret = ClientSecret::new(client_secret.to_string());
    let redirect_url = RedirectUrl::new(redirect_url.to_string())
        .map_err(|_| UtilError::MissingProvider("Invalid redirect URL".to_string()))?;
    let token_url = TokenUrl::new(token_url.to_string())
        .map_err(|_| UtilError::MissingProvider("Invalid token URL".to_string()))?;
    let auth_url = AuthUrl::new(auth_url.to_string())
        .map_err(|_| UtilError::MissingProvider("Invalid auth URL".to_string()))?;
    let redirect_url = RedirectUrl::new(redirect_url.to_string())
        .map_err(|_| UtilError::MissingProvider("Invalid redirect URL".to_string()))?;

    let client = BasicClient::new(client_id)
        .set_auth_uri(auth_url)
        .set_redirect_uri(redirect_url)
        .set_token_uri(token_url)
        .set_client_secret(client_secret);

    Ok(client)
}
