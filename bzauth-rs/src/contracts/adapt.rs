use super::{account::Account, session::Session, user::User};
use crate::tools::awaitable::Awaitable;

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

#[derive(Debug, Clone)]
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

pub struct AdaptVerificationToken {
    pub email: String,
    pub token: String,
    pub expires_in: u64,
}

pub struct UseVerificationTokenOptions {
    pub email: String,
    pub token: String,
}

pub trait Adapt
where
    Self: Send + Sync,
{
    fn create_user(&self, user: AdaptUser) -> Awaitable<AdaptUser>;
    fn get_user(&self, id: String) -> Awaitable<Option<AdaptUser>>;
    fn get_user_by_email(&self, email: String) -> Awaitable<Option<AdaptUser>>;
    fn get_user_by_account(&self, provider: ProviderAccountId) -> Awaitable<Option<AdaptUser>>;
    /// Id is required
    fn update_user(&self, user: AdaptUser) -> Awaitable<AdaptUser>;
    fn delete_user(&self, id: String) -> Awaitable<()>;

    fn get_account(&self, provider: ProviderAccountId) -> Awaitable<Option<AdaptAccount>>;
    fn link_account(&self, account: AdaptAccount) -> Awaitable<Option<AdaptAccount>>;
    fn unlink_account(&self, provider: ProviderAccountId) -> Awaitable<()>;

    fn create_session(&self, options: CreateSessionOptions) -> Awaitable<Option<AdaptSession>>;
    fn get_session_and_user(&self, token: String) -> Awaitable<Option<SessionUser>>;
    /// session_token required
    fn update_session(&self, session: AdaptSession) -> Awaitable<AdaptSession>;
    fn delete_session(&self, token: String) -> Awaitable<()>;

    fn create_verification_token(
        &self,
        token: AdaptVerificationToken,
    ) -> Awaitable<AdaptVerificationToken>;
    fn use_verification_token(
        &self,
        options: UseVerificationTokenOptions,
    ) -> Awaitable<Option<AdaptVerificationToken>>;

    // TODO: Omitting WebAuthn methods for now
}

impl Adapt for Box<dyn Adapt> {
    fn create_user(&self, user: AdaptUser) -> Awaitable<AdaptUser> {
        (**self).create_user(user)
    }

    fn get_user(&self, id: String) -> Awaitable<Option<AdaptUser>> {
        (**self).get_user(id)
    }

    fn get_user_by_email(&self, email: String) -> Awaitable<Option<AdaptUser>> {
        (**self).get_user_by_email(email)
    }

    fn get_user_by_account(&self, provider: ProviderAccountId) -> Awaitable<Option<AdaptUser>> {
        (**self).get_user_by_account(provider)
    }

    fn update_user(&self, user: AdaptUser) -> Awaitable<AdaptUser> {
        (**self).update_user(user)
    }

    fn delete_user(&self, id: String) -> Awaitable<()> {
        (**self).delete_user(id)
    }

    fn get_account(&self, provider: ProviderAccountId) -> Awaitable<Option<AdaptAccount>> {
        (**self).get_account(provider)
    }

    fn link_account(&self, account: AdaptAccount) -> Awaitable<Option<AdaptAccount>> {
        (**self).link_account(account)
    }

    fn unlink_account(&self, provider: ProviderAccountId) -> Awaitable<()> {
        (**self).unlink_account(provider)
    }

    fn create_session(&self, options: CreateSessionOptions) -> Awaitable<Option<AdaptSession>> {
        (**self).create_session(options)
    }

    fn get_session_and_user(&self, token: String) -> Awaitable<Option<SessionUser>> {
        (**self).get_session_and_user(token)
    }

    fn update_session(&self, session: AdaptSession) -> Awaitable<AdaptSession> {
        (**self).update_session(session)
    }

    fn delete_session(&self, token: String) -> Awaitable<()> {
        (**self).delete_session(token)
    }

    fn create_verification_token(
        &self,
        token: AdaptVerificationToken,
    ) -> Awaitable<AdaptVerificationToken> {
        (**self).create_verification_token(token)
    }

    fn use_verification_token(
        &self,
        options: UseVerificationTokenOptions,
    ) -> Awaitable<Option<AdaptVerificationToken>> {
        (**self).use_verification_token(options)
    }
}
