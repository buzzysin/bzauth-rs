pub mod traits;
use diesel::r2d2::{ManageConnection, Pool};
pub use traits::*; // re-export all traits (and macros) from the traits module

use crate::contracts::adapt::{
    Adapt, AdaptAccount, AdaptSession, AdaptUser, AdaptVerificationToken, CreateSessionOptions,
    ProviderAccountId, SessionUser, UseVerificationTokenOptions,
};

pub struct DieselAdapterOptions<
    M,
    Adaptor,
    UserModel,
    AccountModel,
    SessionModel,
    VerificationTokenModel,
> where
    M: ManageConnection,
    Adaptor: AdaptUserOperation<M::Connection, Model = UserModel>,
    Adaptor: AdaptAccountOperation<M::Connection, Model = AccountModel, User = UserModel>,
    Adaptor: AdaptSessionOperation<M::Connection, Model = SessionModel>,
    Adaptor: AdaptVerificationTokenOperation<M::Connection, Model = VerificationTokenModel>,
{
    pub conn_pool: Pool<M>,
    pub adaptor: Adaptor,
}

pub struct DieselAdaptor<M, Adaptor, UserModel, AccountModel, SessionModel, VerificationTokenModel>
where
    M: ManageConnection,
    Adaptor: AdaptUserOperation<M::Connection, Model = UserModel>,
    Adaptor: AdaptAccountOperation<M::Connection, Model = AccountModel, User = UserModel>,
    Adaptor: AdaptSessionOperation<M::Connection, Model = SessionModel>,
    Adaptor: AdaptVerificationTokenOperation<M::Connection, Model = VerificationTokenModel>,
{
    pub options: DieselAdapterOptions<
        M,
        Adaptor,
        UserModel,
        AccountModel,
        SessionModel,
        VerificationTokenModel,
    >,
}

impl<M, Adaptor, UserModel, AccountModel, SessionModel, VerificationTokenModel>
    DieselAdaptor<M, Adaptor, UserModel, AccountModel, SessionModel, VerificationTokenModel>
where
    M: ManageConnection,
    Adaptor: AdaptUserOperation<M::Connection, Model = UserModel>,
    Adaptor: AdaptAccountOperation<M::Connection, Model = AccountModel, User = UserModel>,
    Adaptor: AdaptSessionOperation<M::Connection, Model = SessionModel>,
    Adaptor: AdaptVerificationTokenOperation<M::Connection, Model = VerificationTokenModel>,
{
    pub fn from_options(
        options: DieselAdapterOptions<
            M,
            Adaptor,
            UserModel,
            AccountModel,
            SessionModel,
            VerificationTokenModel,
        >,
    ) -> Self {
        Self { options }
    }
}

#[async_trait::async_trait]
impl<M, Adaptor, UserModel, AccountModel, SessionModel, VerificationTokenModel> Adapt
    for DieselAdaptor<M, Adaptor, UserModel, AccountModel, SessionModel, VerificationTokenModel>
where
    M: ManageConnection,
    AdaptUser: From<UserModel> + 'static,
    UserModel: From<AdaptUser> + 'static,
    Adaptor: AdaptUserOperation<M::Connection, Model = UserModel>,
    AdaptAccount: From<AccountModel> + 'static,
    AccountModel: From<AdaptAccount> + 'static,
    Adaptor: AdaptAccountOperation<M::Connection, Model = AccountModel, User = UserModel>,
    AdaptSession: From<SessionModel> + 'static,
    SessionModel: From<AdaptSession> + 'static,
    Adaptor: AdaptSessionOperation<M::Connection, Model = SessionModel, User = UserModel>,
    AdaptVerificationToken: From<VerificationTokenModel> + 'static,
    VerificationTokenModel: From<AdaptVerificationToken> + 'static,
    Adaptor: AdaptVerificationTokenOperation<M::Connection, Model = VerificationTokenModel>,
{
    async fn create_user(&self, user: AdaptUser) -> AdaptUser {
        // Grab a connection from the pool
        let mut conn = self
            .options
            .conn_pool
            .clone()
            .get()
            .expect("Failed to get connection from pool");

        // Borrow the database adaptor
        let adaptor = &self.options.adaptor;

        // Create the user in the database
        let new_user = adaptor.create_user(&mut conn, &user.into());

        // Return the created user
        AdaptUser::from(new_user)
    }

    async fn get_user(&self, id: String) -> Option<AdaptUser> {
        // Grab a connection from the pool
        let mut conn = self
            .options
            .conn_pool
            .clone()
            .get()
            .expect("Failed to get connection from pool");
        // Borrow the database adaptor
        let adaptor = &self.options.adaptor;
        // Get the user from the database
        let user = adaptor.find_user_by_id(&mut conn, &id);
        // Return the user
        user.map(|user| AdaptUser::from(user))
    }

    async fn get_user_by_email(&self, email: String) -> Option<AdaptUser> {
        // Grab a connection from the pool
        let mut conn = self
            .options
            .conn_pool
            .clone()
            .get()
            .expect("Failed to get connection from pool");
        // Borrow the database adaptor
        let adaptor = &self.options.adaptor;
        // Get the user from the database
        let user = adaptor.find_user_by_email(&mut conn, &email);
        // Return the user
        user.map(|user| AdaptUser::from(user))
    }
    async fn get_user_by_account(&self, provider: ProviderAccountId) -> Option<AdaptUser> {
        // Grab a connection from the pool
        let mut conn = self
            .options
            .conn_pool
            .clone()
            .get()
            .expect("Failed to get connection from pool");
        // Borrow the database adaptor
        let adaptor = &self.options.adaptor;
        // Get the account joined with the user
        let account = adaptor.find_user_by_account(
            &mut conn,
            provider.provider_id,
            provider.provider_account_id,
        );

        // let first = |(a, b)| a;
        let pick_last = |(_, b)| b;

        // Return the user
        account.map(pick_last).map(|user| AdaptUser::from(user))
    }
    /// Id is required
    async fn update_user(&self, user: AdaptUser) -> AdaptUser {
        // Grab a connection from the pool
        let mut conn = self
            .options
            .conn_pool
            .clone()
            .get()
            .expect("Failed to get connection from pool");

        // Borrow the database adaptor
        let adaptor = &self.options.adaptor;
        // Update the user in the database
        let updated_user = adaptor.update_user(&mut conn, &user.into());
        // Return the updated user
        AdaptUser::from(updated_user)
    }
    async fn delete_user(&self, id: String) {
        // Grab a connection from the pool
        let mut conn = self
            .options
            .conn_pool
            .clone()
            .get()
            .expect("Failed to get connection from pool");

        // Borrow the database adaptor
        let adaptor = &self.options.adaptor;

        // Delete the user from the database
        adaptor.delete_user(&mut conn, &id);

        // Return the user
        // AdaptUser::from(updated_user)
    }

    async fn get_account(&self, provider: ProviderAccountId) -> Option<AdaptAccount> {
        // Grab a connection from the pool
        let mut conn = self
            .options
            .conn_pool
            .clone()
            .get()
            .expect("Failed to get connection from pool");
        // Borrow the database adaptor
        let adaptor = &self.options.adaptor;
        // Get the account from the database
        let account = adaptor.find_account_by_id(
            &mut conn,
            provider.provider_id,
            provider.provider_account_id,
        );
        // Return the account
        account.map(|account| AdaptAccount::from(account))
    }

    async fn link_account(&self, account: AdaptAccount) -> Option<AdaptAccount> {
        // Grab a connection from the pool
        let mut conn = self
            .options
            .conn_pool
            .clone()
            .get()
            .expect("Failed to get connection from pool");
        // Borrow the database adaptor
        let adaptor = &self.options.adaptor;
        // Link the account in the database
        let new_account = adaptor.link_account(&mut conn, &account.into());
        // Return the linked account
        Some(AdaptAccount::from(new_account))
    }
    async fn unlink_account(&self, provider: ProviderAccountId) {
        // Grab a connection from the pool
        let mut conn = self
            .options
            .conn_pool
            .clone()
            .get()
            .expect("Failed to get connection from pool");
        // Borrow the database adaptor
        let adaptor = &self.options.adaptor;
        // Unlink the account from the database
        adaptor.unlink_account(
            &mut conn,
            provider.provider_id,
            provider.provider_account_id,
        );
        // Return the account
        // AdaptAccount::from(updated_user)
    }

    async fn create_session(&self, options: CreateSessionOptions) -> Option<AdaptSession> {
        // Grab a connection from the pool
        let mut conn = self
            .options
            .conn_pool
            .clone()
            .get()
            .expect("Failed to get connection from pool");
        // Borrow the database adaptor
        let adaptor = &self.options.adaptor;
        // Create the session in the database
        let new_session = adaptor.create_session(
            &mut conn,
            AdaptSession {
                token: options.token,
                user_id: options.user_id,
                expires_in: options.expires_in,
            }
            .into(),
        );
        // Return the created session
        Some(AdaptSession::from(new_session))
    }
    async fn get_session_and_user(&self, token: String) -> Option<SessionUser> {
        // Grab a connection from the pool
        let mut conn = self
            .options
            .conn_pool
            .clone()
            .get()
            .expect("Failed to get connection from pool");
        // Borrow the database adaptor
        let adaptor = &self.options.adaptor;
        // Get the session from the database
        let session_user = adaptor.find_session_and_user(&mut conn, &token);
        // Return the session and user
        session_user
            .map(|(session, user)| (AdaptSession::from(session), AdaptUser::from(user)))
            .map(|(session, user)| SessionUser { session, user })
    }
    /// session_token required
    async fn update_session(&self, session: AdaptSession) -> AdaptSession {
        // Grab a connection from the pool
        let mut conn = self
            .options
            .conn_pool
            .clone()
            .get()
            .expect("Failed to get connection from pool");
        // Borrow the database adaptor
        let adaptor = &self.options.adaptor;
        // Update the session in the database
        let updated_session = adaptor.update_session(&mut conn, session.into());
        // Return the updated session
        AdaptSession::from(updated_session)
    }
    async fn delete_session(&self, token: String) {
        // Grab a connection from the pool
        let mut conn = self
            .options
            .conn_pool
            .clone()
            .get()
            .expect("Failed to get connection from pool");
        // Borrow the database adaptor
        let adaptor = &self.options.adaptor;
        // Delete the session from the database
        adaptor.delete_session(&mut conn, &token);
        // Return the session
        // AdaptSession::from(updated_session)
    }

    async fn create_verification_token(
        &self,
        token: AdaptVerificationToken,
    ) -> AdaptVerificationToken {
        // Grab a connection from the pool
        let mut conn = self
            .options
            .conn_pool
            .clone()
            .get()
            .expect("Failed to get connection from pool");
        // Borrow the database adaptor
        let adaptor = &self.options.adaptor;
        // Create the verification token in the database
        let new_token = adaptor.create_verification_token(&mut conn, token.into());
        // Return the created verification token
        AdaptVerificationToken::from(new_token)
    }
    async fn use_verification_token(
        &self,
        options: UseVerificationTokenOptions,
    ) -> Option<AdaptVerificationToken> {
        // Grab a connection from the pool
        let mut conn = self
            .options
            .conn_pool
            .clone()
            .get()
            .expect("Failed to get connection from pool");
        // Borrow the database adaptor
        let adaptor = &self.options.adaptor;
        // Use the verification token in the database
        /* let token =  */
        adaptor.use_verification_token(&mut conn, options.email.as_str(), options.token.as_str());
        // Return the used verification token
        // token.map(|token| AdaptVerificationToken::from(token))
        None
    }
}

impl<M, Adaptor, UserModel, AccountModel, SessionModel, VerificationTokenModel>
    From<DieselAdaptor<M, Adaptor, UserModel, AccountModel, SessionModel, VerificationTokenModel>>
    for Box<dyn Adapt>
where
    M: ManageConnection,
    UserModel: From<AdaptUser> + 'static,
    AdaptUser: From<UserModel> + 'static,
    Adaptor: AdaptUserOperation<M::Connection, Model = UserModel>,
    AccountModel: From<AdaptAccount> + 'static,
    AdaptAccount: From<AccountModel> + 'static,
    Adaptor: AdaptAccountOperation<M::Connection, Model = AccountModel, User = UserModel>,
    SessionModel: From<AdaptSession> + 'static,
    AdaptSession: From<SessionModel> + 'static,
    Adaptor: AdaptSessionOperation<M::Connection, Model = SessionModel, User = UserModel>,
    VerificationTokenModel: From<AdaptVerificationToken> + 'static,
    AdaptVerificationToken: From<VerificationTokenModel> + 'static,
    Adaptor: AdaptVerificationTokenOperation<M::Connection, Model = VerificationTokenModel>,
{
    fn from(
        value: DieselAdaptor<
            M,
            Adaptor,
            UserModel,
            AccountModel,
            SessionModel,
            VerificationTokenModel,
        >,
    ) -> Self {
        Box::new(value)
    }
}
