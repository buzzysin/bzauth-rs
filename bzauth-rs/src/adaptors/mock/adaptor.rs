use crate::{
    contracts::adapt::{
        Adapt, AdaptAccount, AdaptSession, AdaptUser, AdaptVerificationToken, CreateSessionOptions,
        ProviderAccountId, SessionUser, UseVerificationTokenOptions,
    },
    tools::awaitable::Awaitable,
};

pub struct MockAdaptor;

impl Adapt for MockAdaptor {
    fn create_user(&self, _user: AdaptUser) -> Awaitable<AdaptUser> {
        unimplemented!()
    }

    fn get_user(&self, _id: String) -> Awaitable<Option<AdaptUser>> {
        unimplemented!()
    }

    fn get_user_by_email(&self, _email: String) -> Awaitable<Option<AdaptUser>> {
        unimplemented!()
    }

    fn get_user_by_account(&self, _provider: ProviderAccountId) -> Awaitable<Option<AdaptUser>> {
        unimplemented!()
    }

    fn update_user(&self, _user: AdaptUser) -> Awaitable<AdaptUser> {
        unimplemented!()
    }

    fn delete_user(&self, _id: String) -> Awaitable<()> {
        unimplemented!()
    }

    fn get_account(&self, _provider: ProviderAccountId) -> Awaitable<Option<AdaptAccount>> {
        unimplemented!()
    }

    fn link_account(&self, _account: AdaptAccount) -> Awaitable<Option<AdaptAccount>> {
        unimplemented!()
    }

    fn unlink_account(&self, _provider: ProviderAccountId) -> Awaitable<()> {
        unimplemented!()
    }

    fn create_session(&self, _options: CreateSessionOptions) -> Awaitable<Option<AdaptSession>> {
        unimplemented!()
    }

    fn get_session_and_user(&self, _token: String) -> Awaitable<Option<SessionUser>> {
        unimplemented!()
    }

    fn update_session(&self, _session: AdaptSession) -> Awaitable<AdaptSession> {
        unimplemented!()
    }

    fn delete_session(&self, _token: String) -> Awaitable<()> {
        unimplemented!()
    }

    fn create_verification_token(
        &self,
        _token: AdaptVerificationToken,
    ) -> Awaitable<AdaptVerificationToken> {
        unimplemented!()
    }

    fn use_verification_token(
        &self,
        _options: UseVerificationTokenOptions,
    ) -> Awaitable<Option<AdaptVerificationToken>> {
        unimplemented!()
    }
}
