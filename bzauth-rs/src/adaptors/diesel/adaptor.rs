use diesel::r2d2::{ManageConnection, Pool};

use super::traits::*;
use crate::{
    awaitable,
    contracts::adapt::{
        Adapt, AdaptAccount, AdaptSession, AdaptUser, AdaptVerificationToken, CreateSessionOptions,
        ProviderAccountId, SessionUser, UseVerificationTokenOptions,
    },
    tools::awaitable::Awaitable,
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

impl<M, Adaptor, UserModel, AccountModel, SessionModel, VerificationTokenModel> Adapt
    for DieselAdaptor<M, Adaptor, UserModel, AccountModel, SessionModel, VerificationTokenModel>
where
    M: ManageConnection,
    AdaptUser: From<UserModel> + 'static,
    UserModel: From<AdaptUser> + Send + 'static,
    Adaptor: AdaptUserOperation<M::Connection, Model = UserModel>,
    AdaptAccount: From<AccountModel> + 'static,
    AccountModel: From<AdaptAccount> + Send + 'static,
    Adaptor: AdaptAccountOperation<M::Connection, Model = AccountModel, User = UserModel>,
    AdaptSession: From<SessionModel> + 'static,
    SessionModel: From<AdaptSession> + Send + 'static,
    Adaptor: AdaptSessionOperation<M::Connection, Model = SessionModel, User = UserModel>,
    AdaptVerificationToken: From<VerificationTokenModel> + 'static,
    VerificationTokenModel: From<AdaptVerificationToken> + Send + 'static,
    Adaptor: AdaptVerificationTokenOperation<M::Connection, Model = VerificationTokenModel>,
{
    fn create_user(&self, user: AdaptUser) -> Awaitable<AdaptUser> {
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
        awaitable!(AdaptUser::from(new_user))
    }

    fn get_user(&self, id: String) -> Awaitable<Option<AdaptUser>> {
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
        awaitable!(user.map(|user| AdaptUser::from(user)))
    }

    fn get_user_by_email(&self, email: String) -> Awaitable<Option<AdaptUser>> {
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
        awaitable!(user.map(|user| AdaptUser::from(user)))
    }
    fn get_user_by_account(&self, provider: ProviderAccountId) -> Awaitable<Option<AdaptUser>> {
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
        awaitable!(account.map(pick_last).map(|user| AdaptUser::from(user)))
    }
    /// Id is required
    fn update_user(&self, user: AdaptUser) -> Awaitable<AdaptUser> {
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
        awaitable!(AdaptUser::from(updated_user))
    }
    fn delete_user(&self, id: String) -> Awaitable<()> {
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
        awaitable!(())
    }

    fn get_account(&self, provider: ProviderAccountId) -> Awaitable<Option<AdaptAccount>> {
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
        awaitable!(account.map(|account| AdaptAccount::from(account)))
    }

    fn link_account(&self, account: AdaptAccount) -> Awaitable<Option<AdaptAccount>> {
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
        awaitable!(Some(AdaptAccount::from(new_account)))
    }
    fn unlink_account(&self, provider: ProviderAccountId) -> Awaitable<()> {
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
        awaitable!(())
    }

    fn create_session(&self, options: CreateSessionOptions) -> Awaitable<Option<AdaptSession>> {
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
        awaitable!(Some(AdaptSession::from(new_session)))
    }
    fn get_session_and_user(&self, token: String) -> Awaitable<Option<SessionUser>> {
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
        awaitable! {
            session_user
                .map(|(session, user)| (AdaptSession::from(session), AdaptUser::from(user)))
                .map(|(session, user)| SessionUser { session, user })
        }
    }
    /// session_token required
    fn update_session(&self, session: AdaptSession) -> Awaitable<AdaptSession> {
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
        awaitable!(AdaptSession::from(updated_session))
    }
    fn delete_session(&self, token: String) -> Awaitable<()> {
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
        awaitable!(())
    }

    fn create_verification_token(
        &self,
        token: AdaptVerificationToken,
    ) -> Awaitable<AdaptVerificationToken> {
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
        awaitable!(AdaptVerificationToken::from(new_token))
    }
    fn use_verification_token(
        &self,
        options: UseVerificationTokenOptions,
    ) -> Awaitable<Option<AdaptVerificationToken>> {
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
        awaitable!(None)
    }
}

impl<M, Adaptor, UserModel, AccountModel, SessionModel, VerificationTokenModel>
    From<DieselAdaptor<M, Adaptor, UserModel, AccountModel, SessionModel, VerificationTokenModel>>
    for Box<dyn Adapt>
where
    M: ManageConnection,
    UserModel: From<AdaptUser> + Send + 'static,
    AdaptUser: From<UserModel> + 'static,
    Adaptor: AdaptUserOperation<M::Connection, Model = UserModel>,
    AccountModel: From<AdaptAccount> + Send + 'static,
    AdaptAccount: From<AccountModel> + 'static,
    Adaptor: AdaptAccountOperation<M::Connection, Model = AccountModel, User = UserModel>,
    SessionModel: From<AdaptSession> + Send + 'static,
    AdaptSession: From<SessionModel> + 'static,
    Adaptor: AdaptSessionOperation<M::Connection, Model = SessionModel, User = UserModel>,
    VerificationTokenModel: From<AdaptVerificationToken> + Send + 'static,
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
