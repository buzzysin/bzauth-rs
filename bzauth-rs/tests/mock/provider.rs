use std::ops::Deref;

use bzauth_rs::contracts::endpoint::Endpoint;
use bzauth_rs::contracts::profile::Profile;
use bzauth_rs::contracts::provide::{ProvideOAuth2, ProviderType, ProvidesProfile};
use bzauth_rs::contracts::user::User;

use crate::mock::consts::{MOCK_AUTHORISE, MOCK_PROFILE, MOCK_TOKEN};

pub const MOCK_PROVIDER_NAME: &str = "MockProvider";
pub const MOCK_PROVIDER_CLIENT_ID: &str = "mock_client_id";
pub const MOCK_PROVIDER_CLIENT_SECRET: &str = "mock_client_secret";
pub const MOCK_PROVIDER_HOST: &str = "localhost";
pub const MOCK_PROVIDER_PORT: u16 = 8081;
pub const MOCK_PROVIDER_URL: &str = "http://localhost:8081";

pub const MOCK_PROVIDER_USER_ID: &str = "mock_user_id";
pub const MOCK_PROVIDER_USER_NAME: &str = "Mock User";
pub const MOCK_PROVIDER_USER_EMAIL: &str = "mock_user@email.com";
pub const MOCK_PROVIDER_USER_IMAGE: &str = "http://placehold.co/500x500";

#[derive(Debug, Clone)]
pub struct MockProvider;

impl ProvideOAuth2 for MockProvider {
    fn id(&self) -> String {
        MOCK_PROVIDER_NAME.to_string()
    }

    fn name(&self) -> String {
        MOCK_PROVIDER_NAME.to_string()
    }

    fn provider_type(&self) -> ProviderType {
        ProviderType::OAuth
    }

    fn client_id(&self) -> String {
        MOCK_PROVIDER_CLIENT_ID.to_string()
    }

    fn client_secret(&self) -> String {
        MOCK_PROVIDER_CLIENT_SECRET.to_string()
    }

    fn auth_endpoint(&self) -> Endpoint {
        format!("{}/{}", MOCK_PROVIDER_URL, MOCK_AUTHORISE).into()
    }

    fn token_endpoint(&self) -> Endpoint {
        format!("{}/{}", MOCK_PROVIDER_URL, MOCK_TOKEN).into()
    }

    fn profile_endpoint(&self) -> Endpoint {
        format!("{}/{}", MOCK_PROVIDER_URL, MOCK_PROFILE).into()
    }
}

pub struct MockProfile(Profile);
impl Deref for MockProfile {
    type Target = Profile;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Profile> for MockProfile {
    fn from(value: Profile) -> Self {
        MockProfile(value)
    }
}

impl ProvidesProfile for MockProvider {
    fn get_profile(&self, profile: Profile) -> Box<User> {
        let mock_profile = MockProfile::from(profile);
        let user = User {
            id: Some(MOCK_PROVIDER_USER_ID.to_string()),
            username: Some(MOCK_PROVIDER_USER_NAME.to_string()),
            email: Some(MOCK_PROVIDER_USER_EMAIL.to_string()),
            image: Some(MOCK_PROVIDER_USER_IMAGE.to_string()),
        };
        Box::new(user)
    }
}
