use serde::{Deserialize, Serialize};

use super::token::Token;
use crate::contracts::provide::ProviderType;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Account {
    /// Unique identifier for the account, may not actually be required
    pub id: Option<String>,
    /// The user ID this account is linked to, if any
    pub user_id: Option<String>,
    /// The provider ID, which is the unique identifier for the account in the provider's system ("discord", "google", etc.)
    pub provider_id: Option<String>,
    /// The type of provider this account is linked to (OAuth, Email, Credentials, OIDC)
    pub provider_type: ProviderType,
    /// The provider account ID, which is the unique identifier for the account in the provider's system
    /// - oauth2: user ID from the provider
    /// - email: email address
    /// - credentials: username (e.g., might be something else)
    pub provider_account_id: Option<String>,
    /// The token associated with this account, if any
    pub token: Option<Token>,
}
