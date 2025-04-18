use std::ops::Deref;

use super::{Account, Session, User};

pub struct AdaptUser {
    pub user: User,
}
impl From<User> for AdaptUser {
    fn from(user: User) -> Self {
        AdaptUser { user }
    }
}
impl From<AdaptUser> for User {
    fn from(adapt_user: AdaptUser) -> Self {
        adapt_user.user
    }
}
impl Deref for AdaptUser {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.user
    }
}

pub struct ProviderAccountId {
    pub provider_id: String,
    pub provider_account_id: String,
}

pub struct AdaptAccount {
    pub account: Account,
}
impl From<Account> for AdaptAccount {
    fn from(account: Account) -> Self {
        AdaptAccount { account }
    }
}
impl From<AdaptAccount> for Account {
    fn from(adapt_account: AdaptAccount) -> Self {
        adapt_account.account
    }
}

pub struct CreateSessionOptions {
    pub token: String,
    pub user_id: String,
    pub expires_in: u64,
}

pub struct SessionUser {
    pub session: AdaptSession,
    pub user: AdaptUser,
}

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
                session.expires_at.unwrap_or(0) as u64 - now
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
    async fn delete_user(&self, id: String);

    async fn get_account(&self, provider: ProviderAccountId) -> Option<AdaptAccount>;
    async fn link_account(&self, account: AdaptAccount) -> Option<AdaptAccount>;
    async fn unlink_account(&self, provider: ProviderAccountId);

    async fn create_session(&self, options: CreateSessionOptions) -> Option<AdaptSession>;
    async fn get_session_and_user(&self, token: String) -> Option<SessionUser>;
    /// session_token required
    async fn update_session(&self, session: AdaptSession) -> AdaptSession;
    async fn delete_session(&self, token: String);

    async fn create_verification_token(
        &self,
        token: AdaptVerificationToken,
    ) -> AdaptVerificationToken;
    async fn use_verification_token(
        &self,
        options: UseVerificationTokenOptions,
    ) -> Option<AdaptVerificationToken>;

    // TODO: Omitting WebAuthn methods for now
}
