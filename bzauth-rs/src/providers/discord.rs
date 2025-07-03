use std::collections::HashMap;

use super::error::ProviderError;
use crate::contracts::{
    endpoint::Endpoint,
    profile::Profile,
    provide::{ProvideOAuth2, ProviderType, ProvidesProfile},
    user::User,
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
    profile_endpoint: Endpoint,
    profile_resolver: fn(profile: DiscordProfile) -> Box<User>,
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
                    map.insert(String::from("scope"), String::from("identify+email"));
                    map
                },
            )),
            token_endpoint: "https://discord.com/api/oauth2/token".into(),
            profile_endpoint: "https://discord.com/api/users/@me".into(),
            profile_resolver: |profile| {
                let mut profile = profile;
                profile.image_url = derive_avatar_image(&profile);

                Box::new(User {
                    id: Some(profile.id),
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

fn derive_avatar_image(profile: &DiscordProfile) -> Option<String> {
    if let Some(avatar) = &profile.avatar {
        // If the avatar starts with "a_", it's an animated avatar (GIF)
        let extension = if avatar.starts_with("a_") {
            "gif"
        }
        // Otherwise, it's a static avatar (PNG)
        else {
            "png"
        };

        Some(format!(
            "https://cdn.discordapp.com/avatars/{}/{}.{}?size={}",
            profile.id, avatar, extension, 1024
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
    fn profile_endpoint(&self) -> Endpoint {
        self.profile_endpoint.clone()
    }
}

impl From<Profile> for DiscordProfile {
    fn from(value: Profile) -> Self {
        let id = value.id.unwrap();

        let username = value
            .others
            .get("username")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let discriminator = value
            .others
            .get("discriminator")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        let avatar = value
            .others
            .get("avatar")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let email = value.email;

        DiscordProfile {
            id,
            username,
            discriminator,
            avatar,
            image_url: None, // Will be set later
            email,
        }
    }
}

impl ProvidesProfile for DiscordProvider {
    fn get_profile(&self, profile: Profile) -> Box<User> {
        (self.profile_resolver)(profile.into())
    }
}
