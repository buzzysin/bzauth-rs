use serde::{Deserialize, Serialize};

use super::account::Account;
use super::session::Session;
use super::user::User;

pub type AdaptUser = User;

pub struct ProviderAccountId {
    pub provider_id: String,
    pub provider_account_id: String,
}

pub type AdaptAccount = Account;

#[derive(Debug, Clone)]
pub struct CreateSessionOptions {
    pub token: String,
    pub user_id: String,
    pub expires_in: u64,
}

#[derive(Debug, Clone)]
pub struct SessionUser {
    pub session: AdaptSession,
    pub user: AdaptUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptSession {
    pub token: String,
    pub user_id: String,
    pub expires_in: u64,
}
impl AdaptSession {
    pub fn adapt_from(session: Session, token: String) -> Self {
        AdaptSession {
            token,
            user_id: session.user.unwrap().id.unwrap(),
            expires_in: {
                let now = chrono::Utc::now().timestamp() as u64;
                session.expires_at.unwrap_or(0) - now
            },
        }
    }
    pub fn adapt_into(&self, session: &Session) -> Session {
        Session {
            user: Some(User {
                id: Some(self.user_id.clone()),
                ..session.user.clone().unwrap()
            }),
            expires_at: {
                let now = chrono::Utc::now().timestamp() as u64;
                Some(self.expires_in + now)
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptVerificationToken {
    pub email: String,
    pub token: String,
    pub expires_in: u64,
}

pub struct UseVerificationTokenOptions {
    pub email: String,
    pub token: String,
}

#[async_trait::async_trait]
pub trait Adapt
where
    Self: Send + Sync,
{
    async fn create_user(&self, user: AdaptUser) -> AdaptUser;
    async fn get_user(&self, id: String) -> Option<AdaptUser>;
    async fn get_user_by_email(&self, email: String) -> Option<AdaptUser>;
    async fn get_user_by_account(&self, provider: ProviderAccountId) -> Option<AdaptUser>;
    /// Id is required
    async fn update_user(&self, user: AdaptUser) -> AdaptUser;
    async fn delete_user(&self, id: String) -> ();

    async fn get_account(&self, provider: ProviderAccountId) -> Option<AdaptAccount>;
    async fn link_account(&self, account: AdaptAccount) -> Option<AdaptAccount>;
    async fn unlink_account(&self, provider: ProviderAccountId) -> ();

    async fn create_session(&self, options: CreateSessionOptions) -> Option<AdaptSession>;
    async fn get_session_and_user(&self, token: String) -> Option<SessionUser>;
    /// session_token required
    async fn update_session(&self, session: AdaptSession) -> AdaptSession;
    async fn delete_session(&self, token: String) -> ();

    fn create_verification_token(&self, token: AdaptVerificationToken) -> AdaptVerificationToken;
    fn use_verification_token(
        &self,
        options: UseVerificationTokenOptions,
    ) -> Option<AdaptVerificationToken>;

    // TODO: Omitting WebAuthn methods for now
}

// impl Adapt for Box<dyn Adapt> {
//     async fn create_user(&self, user: AdaptUser) -> AdaptUser {
//         (**self).create_user(user)
//     }

//     async fn get_user(&self, id: String) -> Option<AdaptUser> {
//         (**self).get_user(id)
//     }

//     async fn get_user_by_email(&self, email: String) -> Option<AdaptUser> {
//         (**self).get_user_by_email(email)
//     }

//     async fn get_user_by_account(&self, provider: ProviderAccountId) -> Option<AdaptUser> {
//         (**self).get_user_by_account(provider)
//     }

//     async fn update_user(&self, user: AdaptUser) -> AdaptUser {
//         (**self).update_user(user)
//     }

//     async fn delete_user(&self, id: String) -> () {
//         (**self).delete_user(id);
//     }

//     async fn get_account(&self, provider: ProviderAccountId) -> Option<AdaptAccount> {
//         (**self).get_account(provider)
//     }

//     async fn link_account(&self, account: AdaptAccount) -> Option<AdaptAccount> {
//         (**self).link_account(account)
//     }

//     async fn unlink_account(&self, provider: ProviderAccountId) -> () {
//         (**self).unlink_account(provider);
//     }

//     async fn create_session(&self, options: CreateSessionOptions) -> Option<AdaptSession> {
//         (**self).create_session(options)
//     }

//     async fn get_session_and_user(&self, token: String) -> Option<SessionUser> {
//         (**self).get_session_and_user(token)
//     }

//     async fn update_session(&self, session: AdaptSession) -> AdaptSession {
//         (**self).update_session(session)
//     }

//     async fn delete_session(&self, token: String) -> () {
//         (**self).delete_session(token);
//     }

//     fn create_verification_token(&self, token: AdaptVerificationToken) -> AdaptVerificationToken {
//         (**self).create_verification_token(token)
//     }

//     fn use_verification_token(
//         &self,
//         options: UseVerificationTokenOptions,
//     ) -> Option<AdaptVerificationToken> {
//         (**self).use_verification_token(options)
//     }
// }
