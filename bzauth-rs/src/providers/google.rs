use std::collections::HashMap;

use super::error::ProviderError;
use crate::contracts::{
    User,
    endpoint::Endpoint,
    provide::{ProvideOAuth2, ProviderType},
};

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
    pub fn new() -> Self {
        Self::from_options(GoogleProviderOptions {
            client_id: std::env::var("GOOGLE_CLIENT_ID").ok(),
            client_secret: std::env::var("GOOGLE_CLIENT_SECRET").ok(),
            ..Default::default()
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
            id: "Google".to_string(),
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
                let profile = profile;

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
    fn userinfo_endpoint(&self) -> Endpoint {
        self.userinfo_endpoint.clone()
    }
}

//
// fn get_profile(&self) -> fn(GoogleProfile) -> Box<User> {
//     self.profile
// }

// Allows implicit conversion from GoogleProvider to Box<dyn Provide>
// impl From<GoogleProvider> for Box<dyn Provide> {
//     fn from(provider: GoogleProvider) -> Self {
//         Box::new(provider)
//     }
// }
