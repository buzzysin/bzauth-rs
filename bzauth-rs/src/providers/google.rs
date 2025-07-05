use std::collections::HashMap;

use super::error::ProviderError;
use crate::contracts::endpoint::Endpoint;
use crate::contracts::profile::Profile;
use crate::contracts::provide::{ProvideOAuth2, ProviderType, ProvidesProfile};
use crate::contracts::user::User;

#[derive(Debug, Clone, Default)]
pub struct GoogleProfile {
    pub aud: String,
    pub azp: String,
    pub email: String,
    pub email_verified: bool,
    pub exp: i64,
    pub family_name: String,
    pub given_name: String,
    pub hd: String,
    pub iat: i64,
    pub iss: String,
    pub name: String,
    pub picture: String,
    pub sub: String,
}

#[derive(Debug, Clone)]
pub struct GoogleProvider {
    id: String,
    name: String,
    provider_type: ProviderType,
    client_id: String,
    client_secret: String,
    auth_endpoint: Endpoint,
    token_endpoint: Endpoint,
    userinfo_endpoint: Endpoint,
    _profile: fn(profile: GoogleProfile) -> Box<User>,
    _options: GoogleProviderOptions,
}

#[derive(Debug, Clone, Default)]
pub struct GoogleProviderOptions {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

impl GoogleProvider {
    /// Create a new GoogleProvider with default options
    ///
    /// This will use the environment variables GOOGLE_CLIENT_ID and GOOGLE_CLIENT_SECRET
    pub fn new() -> Self {
        Self::from_options(GoogleProviderOptions {
            client_id: std::env::var("GOOGLE_CLIENT_ID").ok(),
            client_secret: std::env::var("GOOGLE_CLIENT_SECRET").ok(),
        })
        .unwrap()
    }

    pub fn from_options(options: GoogleProviderOptions) -> Result<Self, ProviderError> {
        let client_id = options
            .clone()
            .client_id
            .ok_or(ProviderError::MissingClientId("".to_string()))?;
        let client_secret = options
            .clone()
            .client_secret
            .ok_or(ProviderError::MissingClientSecret("".to_string()))?;

        let provider = GoogleProvider {
            id: "google".to_string(),
            name: "Google".to_string(),
            provider_type: ProviderType::OAuth,
            client_id,
            client_secret,
            auth_endpoint: Endpoint::from((
                String::from("https://accounts.google.com/o/oauth2/v2/auth"),
                {
                    let mut map = HashMap::<String, String>::new();
                    map.insert(String::from("scope"), String::from("openid email profile"));
                    map
                },
            )),
            token_endpoint: "https://oauth2.googleapis.com/token".into(),
            userinfo_endpoint: "https://openidconnect.googleapis.com/v1/userinfo".into(),
            _profile: |profile| {
                Box::new(User {
                    // todo.
                    id: Some(profile.sub),
                    username: Some(profile.given_name),
                    email: Some(profile.email),
                    image: Some(profile.picture),
                })
            },
            _options: options,
        };

        Ok(provider)
    }
}

impl Default for GoogleProvider {
    /// Create a new GoogleProvider with default options
    fn default() -> Self {
        Self::new()
    }
}

impl ProvideOAuth2 for GoogleProvider {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn provider_type(&self) -> ProviderType {
        self.provider_type.clone()
    }

    fn client_id(&self) -> String {
        self.client_id.clone()
    }

    fn client_secret(&self) -> String {
        self.client_secret.clone()
    }

    // Endpoints
    fn auth_endpoint(&self) -> Endpoint {
        self.auth_endpoint.clone()
    }
    fn token_endpoint(&self) -> Endpoint {
        self.token_endpoint.clone()
    }
    fn profile_endpoint(&self) -> Endpoint {
        self.userinfo_endpoint.clone()
    }
}

impl From<Profile> for GoogleProfile {
    #[allow(unused_variables)]
    fn from(value: Profile) -> Self {
        // Pick from others: aud, azp, exp, hd, iat, iss
        let aud = value.others.get("aud").cloned().unwrap_or_default();
        let azp = value.others.get("azp").cloned().unwrap_or_default();
        let exp = value
            .others
            .get("exp")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        let iat = value
            .others
            .get("iat")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        let hd = value.others.get("hd").cloned().unwrap_or_default();
        let iss = value.others.get("iss").cloned().unwrap_or_default();

        GoogleProfile {
            // aud: value.aud,
            // azp: value.azp,
            email: value.email.unwrap(),
            email_verified: value.email_verified.unwrap_or(false),
            // exp: value.exp,
            family_name: value.family_name.unwrap_or_default(),
            given_name: value.given_name.unwrap_or_default(),
            // hd: value.hd.unwrap_or_default(),
            // iat: value.iat,
            // iss: value.iss,
            name: value.name.unwrap_or_default(),
            picture: value.picture.unwrap_or_default(),
            sub: value.sub.unwrap_or_default(),
            ..Default::default()
        }
    }
}

impl ProvidesProfile for GoogleProvider {
    fn get_profile(&self, profile: Profile) -> Box<User> {
        (self._profile)(profile.into())
    }
}
