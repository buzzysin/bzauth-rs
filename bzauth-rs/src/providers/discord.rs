use std::collections::HashMap;

use super::error::ProviderError;
use crate::contracts::{
    User,
    endpoint::Endpoint,
    provide::{ProvideOAuth2, ProviderType},
};

pub struct DiscordProfile {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub image_url: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DiscordProvider {
    id: String,
    name: String,
    provider_type: ProviderType,
    client_id: String,
    client_secret: String,
    auth_endpoint: Endpoint,
    token_endpoint: Endpoint,
    userinfo_endpoint: Endpoint,
    _profile: fn(profile: DiscordProfile) -> Box<User>,
    _options: DiscordProviderOptions,
}

#[derive(Debug, Clone, Default)]
pub struct DiscordProviderOptions {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

impl DiscordProvider {
    /// Create a new DiscordProvider with default options
    ///
    /// This will use the environment variables DISCORD_CLIENT_ID and DISCORD_CLIENT_SECRET
    pub fn new() -> Self {
        let client_id = std::env::var("DISCORD_CLIENT_ID").ok();
        let client_secret = std::env::var("DISCORD_CLIENT_SECRET").ok();

        Self::from_options(DiscordProviderOptions {
            client_id,
            client_secret,
        })
        .unwrap()
    }

    pub fn from_options(options: DiscordProviderOptions) -> Result<Self, ProviderError> {
        let client_id = options
            .clone()
            .client_id
            .ok_or(ProviderError::MissingClientId("".to_string()))?;
        let client_secret = options
            .clone()
            .client_secret
            .ok_or(ProviderError::MissingClientSecret("".to_string()))?;

        let provider = DiscordProvider {
            id: "discord".to_string(),
            name: "Discord".to_string(),
            provider_type: ProviderType::OAuth,
            client_id,
            client_secret,
            auth_endpoint: Endpoint::from((
                String::from("https://discord.com/oauth2/authorize"),
                {
                    let mut map = HashMap::<String, String>::new();
                    map.insert(String::from("scope"), String::from("identify email"));
                    map
                },
            )),
            token_endpoint: "https://discord.com/api/oauth2/token".into(),
            userinfo_endpoint: "https://discord.com/api/oauth2/token".into(),
            _profile: |profile| {
                let mut profile = profile;

                profile.image_url = if let Some(avatar) = profile.avatar {
                    let extension = if avatar.starts_with("a_") {
                        "gif"
                    } else {
                        "png"
                    };

                    Some(format!(
                        "https://cdn.discordapp.com/embed/avatars/{}.{}",
                        profile.id, extension
                    ))
                } else {
                    let default_avatar_number = if profile.discriminator == "0" {
                        profile.id.parse::<u32>().unwrap() >> 22
                    } else {
                        profile.discriminator.parse::<u32>().unwrap() % 5
                    };

                    Some(format!(
                        "https://cdn.discordapp.com/embed/avatars/{}.png",
                        default_avatar_number
                    ))
                };

                Box::new(User {
                    id: Some("".to_string()),
                    username: Some(profile.username),
                    email: profile.email,
                    image: profile.image_url,
                })
            },
            _options: options,
        };

        Ok(provider)
    }
}

impl Default for DiscordProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ProvideOAuth2 for DiscordProvider {
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

// impl

// fn get_profile(&self) -> fn(DiscordProfile) -> Box<User> {
//     self.profile
// }

// Allows implicit conversion from DiscordProvider to Box<dyn Provide>
// impl From<DiscordProvider> for Box<dyn Provide> {
//     fn from(provider: DiscordProvider) -> Self {
//         Box::new(provider)
//     }
// }
