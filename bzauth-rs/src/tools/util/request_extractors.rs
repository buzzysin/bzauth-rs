use std::sync::Arc;

use super::generators::{Oauth2Client, generate_client_from_auth};
use crate::auth::Auth;
use crate::contracts::adapt::Adapt;
use crate::contracts::provide::Provide;
use crate::tools::CoreError;
use crate::tools::request::CoreRequest;
use crate::tools::response::RequestPayload;

pub enum UtilError {
    MissingAuth(String),
    MissingProviderId(String),
    MissingProvider(String),
    //
    ClientCreationFailed(String),
}

impl From<UtilError> for CoreError {
    fn from(error: UtilError) -> Self {
        match error {
            UtilError::MissingAuth(msg) => CoreError::new().with_message(msg),
            UtilError::MissingProviderId(msg) => CoreError::new().with_message(msg),
            UtilError::MissingProvider(msg) => CoreError::new().with_message(msg),
            UtilError::ClientCreationFailed(msg) => CoreError::new().with_message(msg),
        }
    }
}

/// Extends the CoreRequest object
impl<T: RequestPayload> CoreRequest<T> {
    /// Extracts the auth object from the request.
    pub fn extract_auth(&self) -> Result<Arc<Auth>, UtilError> {
        self.auth()
            .ok_or_else(|| UtilError::MissingAuth("Auth not found".to_string()))
            .cloned()
    }

    /// Extracts the provider ID from the request path.
    pub fn extract_provider_id(&self) -> Result<String, UtilError> {
        self.path()
            .split('/')
            .nth(2)
            .ok_or_else(|| UtilError::MissingProviderId("Provider ID not found".to_string()))
            .map(|id| id.to_string())
    }

    /// Extracts the provider from the request based on the provider ID and auth options.
    pub fn extract_provider(&self) -> Result<Box<dyn Provide>, UtilError> {
        let provider_id = self.extract_provider_id()?;
        let auth = self.extract_auth()?;

        let provider = auth
            .options
            .providers
            .iter()
            .find(|p| p.id() == provider_id)
            .ok_or_else(|| UtilError::MissingProvider("Provider not found".to_string()))?
            .clone();

        Ok(provider)
    }

    pub fn extract_adaptor(&self) -> Result<&dyn Adapt, UtilError> {
        let adaptor = self
            .auth()
            .ok_or_else(|| UtilError::MissingAuth("Auth not found".to_string()))?
            .adaptor()
            .ok_or_else(|| UtilError::MissingAuth("Adaptor not found".to_string()))?;

        Ok(adaptor)
    }

    /// Extracts the authorization code from the request query parameters.
    pub fn extract_code(&self) -> Result<String, UtilError> {
        let code_from_query = self
            .query()
            .get("code")
            .ok_or_else(|| UtilError::MissingAuth("Missing authorization code".to_string()))?
            .to_string();

        Ok(code_from_query)
    }

    /// Extracts the state from the request query parameters.
    pub fn extract_state(&self) -> Result<String, UtilError> {
        let state = self
            .query()
            .get("state")
            .ok_or_else(|| UtilError::MissingAuth("Missing state".to_string()))?
            .to_string();

        Ok(state)
    }

    /// Extracts the OAuth2 client from the request.
    pub fn extract_oauth2_client(&self) -> Result<Oauth2Client, UtilError> {
        let provider = self.extract_provider()?;
        let oauth2_provider = provider
            .as_ref()
            .as_oauth2()
            .ok_or_else(|| UtilError::MissingProvider("Provider is not OAuth2".to_string()))?;

        generate_client_from_auth(oauth2_provider)
    }
}

pub const COOKIE_STATE: &str = "state";
pub const COOKIE_CSRF_TOKEN: &str = "csrf";
pub const COOKIE_PKCE: &str = "pkce";
pub const COOKIE_PKCE_METHOD: &str = "pkce_method";
pub const COOKIE_PKCE_VERIFIER: &str = "pkce_verifier";
