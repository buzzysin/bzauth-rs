use crate::contracts::{account::Account, adapt::Adapt, provide::Provide, user::User};

pub struct SignInOptions {
    pub user: Box<User>,
    pub account: Option<Box<Account>>,
    pub profile: Option<Box<User>>,
}

pub enum SignInResult {
    Success,
    Redirect(String),
    Error(String),
}

#[derive(Clone, Default)]
pub struct AuthCallbackOptions {
    pub sign_in: Option<fn(SignInOptions) -> dyn Future<Output = SignInResult>>,
}

#[derive(Clone, Default)]
pub struct AuthSessionOptions {
    pub strategy: Option<String>,
    pub max_age: Option<i64>,
    pub update_age: Option<i64>,
    pub generate_session: Option<fn() -> String>,
}

#[derive(Default)]
pub struct AuthOptions {
    pub providers: Vec<Box<dyn Provide>>,
    pub adaptor: Option<Box<dyn Adapt>>,
    pub callbacks: Option<AuthCallbackOptions>,
    pub session: Option<AuthSessionOptions>,
}

impl AuthOptions {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_provider(self, provider: Box<dyn Provide>) -> Self {
        let mut providers = self.providers;
        providers.push(provider);

        Self { providers, ..self }
    }
    pub fn with_providers(self, providers: Vec<Box<dyn Provide>>) -> Self {
        Self { providers, ..self }
    }
    pub fn with_adaptor(self, adaptor: Box<dyn Adapt>) -> Self {
        Self {
            adaptor: Some(adaptor),
            ..self
        }
    }
    pub fn with_callback(
        self,
        callback: fn(SignInOptions) -> dyn Future<Output = SignInResult>,
    ) -> Self {
        let mut callbacks = self.callbacks.unwrap_or_default();
        callbacks.sign_in = Some(callback);
        Self {
            callbacks: Some(callbacks),
            ..self
        }
    }
    pub fn with_callbacks(self, callbacks: AuthCallbackOptions) -> Self {
        Self {
            callbacks: Some(callbacks),
            ..self
        }
    }
    pub fn with_session(self, session: AuthSessionOptions) -> Self {
        Self {
            session: Some(session),
            ..self
        }
    }
}

pub struct Auth {
    pub options: AuthOptions,
}

impl Auth {
    pub fn from_options(options: AuthOptions) -> Self {
        Self { options }
    }
}
