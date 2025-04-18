use std::collections::HashMap;

pub mod adapt;
pub mod endpoint;
pub mod provide;

#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Token {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
    pub others: HashMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct Account {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub provider_id: Option<String>,
    pub token: Option<Token>,
}

/// Represents the profile of a user, as returned by an OAuth2 provider. Not every field is guaranteed to be present.
/// https://openid.net/specs/openid-connect-core-1_0.html#StandardClaims
/// Note: GDPR warning - a lot of these fields are considered personal data.
#[derive(Debug, Clone, Default)]
pub struct Profile {
    pub id: Option<String>,
    pub sub: Option<String>, // Subject - Identifier for the End-User at the Issuer.
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub middle_name: Option<String>,
    pub nickname: Option<String>,
    pub preferred_username: Option<String>,
    pub profile: Option<String>,
    pub picture: Option<String>,
    pub website: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub gender: Option<String>,
    pub birthdate: Option<String>,
    pub zoneinfo: Option<String>,
    pub locale: Option<String>,
    pub phone_number: Option<String>,
    pub updated_at: Option<u64>,
    pub address: Option<ProfileAddress>,
    pub others: HashMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct ProfileAddress {
    pub formatted: Option<String>,
    pub street_address: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Session {
    pub user: Option<User>,
    pub expires_at: Option<u64>,
}
