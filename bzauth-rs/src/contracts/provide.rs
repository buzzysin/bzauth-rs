use std::any::Any;

use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};

use super::endpoint::Endpoint;
use super::profile::Profile;
use super::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ProviderType {
    #[serde(rename = "oidc")]
    OIDC,
    #[serde(rename = "oauth")]
    #[default]
    OAuth,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "credentials")]
    Credentials,
}

impl From<ProviderType> for String {
    fn from(provider_type: ProviderType) -> Self {
        match provider_type {
            ProviderType::OIDC => "oidc".to_string(),
            ProviderType::OAuth => "oauth".to_string(),
            ProviderType::Email => "email".to_string(),
            ProviderType::Credentials => "credentials".to_string(),
        }
    }
}

impl TryFrom<String> for ProviderType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "oidc" => Ok(ProviderType::OIDC),
            "oauth" => Ok(ProviderType::OAuth),
            "email" => Ok(ProviderType::Email),
            "credentials" => Ok(ProviderType::Credentials),
            _ => Err(format!("Unknown provider type: {}", value)),
        }
    }
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

impl Provide for Box<dyn Provide> {
    fn id(&self) -> String {
        self.as_ref().id()
    }

    fn name(&self) -> String {
        self.as_ref().name()
    }

    fn provider_type(&self) -> ProviderType {
        self.as_ref().provider_type()
    }

    fn as_any(&self) -> &dyn Any {
        self.as_ref().as_any()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self.as_mut().as_any_mut()
    }

    fn as_oauth2(&self) -> Option<&dyn ProvideOAuth2> {
        self.as_ref().as_oauth2()
    }
}

pub trait ProvideOAuth2: Send + Sync + Any + ProvidesProfile + 'static
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
    fn profile_endpoint(&self) -> Endpoint;
}
dyn_clone::clone_trait_object!(ProvideOAuth2);

pub trait ProvidesProfile: Send + Sync + Any + 'static
where
    Self: DynClone,
{
    fn get_profile(&self, profile: Profile) -> Box<User>;
}
dyn_clone::clone_trait_object!(ProvidesProfile);

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

// impl<T: Provide> From<T> for Box<dyn Provide> {
//     fn from(provider: T) -> Self {
//         Box::new(provider)
//     }
// }
