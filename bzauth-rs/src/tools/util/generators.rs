use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl, TokenUrl};
use rand::Rng;
use rand::distr::Alphanumeric;

use super::request_extractors::UtilError;
use crate::contracts::provide::ProvideOAuth2;

pub(crate) type Oauth2Client =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

pub fn generate_client_from_auth(
    oauth2_provider: &dyn ProvideOAuth2,
) -> Result<Oauth2Client, UtilError> {
    let auth_url = oauth2_provider.auth_endpoint().url();
    let client_id = oauth2_provider.client_id();
    let client_secret = oauth2_provider.client_secret();

    // This should be the redirect URL you set in your provider settings
    let redirect_url = format!(
        "http://localhost:3001/auth/callback/{provider}",
        provider = oauth2_provider.id()
    );

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
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url)
        .set_client_secret(client_secret);

    Ok(client)
}

pub fn generate_state() -> String {
    // Generate 32 random alphanumeric characters
    // Using rand::thread_rng() for a cryptographically secure random number generator.
    rand::rng()
        .sample_iter(&Alphanumeric) // Sample characters from the Alphanumeric distribution
        .take(32) // Take 32 characters
        .map(char::from) // Convert bytes to chars
        .collect() // Collect into a String
}

pub fn generate_http_client() -> Result<reqwest::Client, UtilError> {
    reqwest::Client::builder()
        .build()
        .map_err(|_| UtilError::MissingProvider("Failed to create HTTP client".to_string()))
}
