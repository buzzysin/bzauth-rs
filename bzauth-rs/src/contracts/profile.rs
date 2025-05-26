use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Represents the profile of a user, as returned by an OAuth2 provider. Not every field is guaranteed to be present.
/// https://openid.net/specs/openid-connect-core-1_0.html#StandardClaims
/// Note: GDPR warning - a lot of these fields are considered personal data.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ProfileAddress {
    pub formatted: Option<String>,
    pub street_address: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
}
