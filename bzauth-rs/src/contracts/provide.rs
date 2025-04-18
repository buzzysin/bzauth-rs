use std::any::Any;

use dyn_clone::DynClone;
use serde::Serialize;

use super::{User, endpoint::Endpoint};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum ProviderType {
    OIDC,
    OAuth,
    Email,
    Credentials,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderType::OIDC => write!(f, "oidc"),
            ProviderType::OAuth => write!(f, "oauth"),
            ProviderType::Email => write!(f, "email"),
            ProviderType::Credentials => write!(f, "credentials"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProviderOAuth2Check {
    None,
    State,
    PKCE,
}

pub trait Provide: Send + Sync + Any + 'static
where
    Self: DynClone,
{
    fn id(&self) -> String;
    fn name(&self) -> String;
    fn provider_type(&self) -> ProviderType;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_oauth2(&self) -> Option<&dyn ProvideOAuth2> {
        None
    }
}
dyn_clone::clone_trait_object!(Provide);

pub trait ProvideOAuth2: Send + Sync + Any + 'static
where
    Self: DynClone,
{
    fn id(&self) -> String;
    fn name(&self) -> String;
    fn provider_type(&self) -> ProviderType;
    fn client_id(&self) -> String; // Required
    fn client_secret(&self) -> String; // Required

    // Endpoints
    fn auth_endpoint(&self) -> Endpoint;
    fn token_endpoint(&self) -> Endpoint;
    fn userinfo_endpoint(&self) -> Endpoint;
}
dyn_clone::clone_trait_object!(ProvideOAuth2);

pub trait ProvidesProfile: Send + Sync + Any + 'static
where
    Self: DynClone,
{
    type Profile;

    fn get_profile(&self) -> fn(Self::Profile) -> Box<User>;
}
dyn_clone::clone_trait_object!(<Profile> ProvidesProfile<Profile = Profile>);

// dyn_clone::clone_trait_object!(ProvideOAuth2);

impl<T: ProvideOAuth2> Provide for T {
    fn id(&self) -> String {
        self.id()
    }

    fn name(&self) -> String {
        self.name()
    }

    fn provider_type(&self) -> ProviderType {
        self.provider_type()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_oauth2(&self) -> Option<&dyn ProvideOAuth2> {
        Some(self)
    }
}

impl<T: Provide> From<T> for Box<dyn Provide> {
    fn from(provider: T) -> Self {
        Box::new(provider)
    }
}
