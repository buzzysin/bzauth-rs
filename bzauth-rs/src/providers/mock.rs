use crate::contracts::{
    endpoint::Endpoint,
    profile::Profile,
    provide::{ProvideOAuth2, ProviderType, ProvidesProfile},
    user::User,
};

#[derive(Debug, Clone)]
pub struct MockOauthProvider;

impl ProvideOAuth2 for MockOauthProvider {
    fn id(&self) -> String {
        "mock_oauth".to_string()
    }

    fn name(&self) -> String {
        "Mock OAuth Provider".to_string()
    }

    fn provider_type(&self) -> ProviderType {
        ProviderType::OAuth
    }

    fn client_id(&self) -> String {
        "mock_client_id".to_string()
    }

    fn client_secret(&self) -> String {
        "mock_client_secret".to_string()
    }

    fn auth_endpoint(&self) -> Endpoint {
        "http://mock.auth.endpoint".into()
    }
    fn token_endpoint(&self) -> Endpoint {
        "http://mock.token.endpoint".into()
    }
    fn profile_endpoint(&self) -> Endpoint {
        "http://mock.profile.endpoint".into()
    }
}

impl ProvidesProfile for MockOauthProvider {
    fn get_profile(&self, profile: Profile) -> Box<User> {
        Box::new(User {
            id: profile.id,
            username: profile.sub,
            email: profile.email,
            image: profile.picture,
        })
    }
}
