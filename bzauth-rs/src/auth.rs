use crate::contracts::{Account, User, adapt::Adapt, provide::Provide};

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
    pub fn add_provider(mut self, provider: Box<dyn Provide>) -> Self {
        self.providers.push(provider);
        self
    }
    pub fn with_providers(mut self, providers: Vec<Box<dyn Provide>>) -> Self {
        self.providers = providers;
        self
    }
    pub fn with_adaptor(mut self, adaptor: Box<dyn Adapt>) -> Self {
        self.adaptor = Some(adaptor);
        self
    }
    pub fn with_callback(
        mut self,
        callback: fn(SignInOptions) -> dyn Future<Output = SignInResult>,
    ) -> Self {
        if let Some(ref mut callbacks) = self.callbacks {
            callbacks.sign_in = Some(callback);
        } else {
            self.callbacks = Some(AuthCallbackOptions {
                sign_in: Some(callback),
            });
        }
        self
    }
    pub fn with_callbacks(mut self, callbacks: AuthCallbackOptions) -> Self {
        self.callbacks = Some(callbacks);
        self
    }
    pub fn with_session(mut self, session: AuthSessionOptions) -> Self {
        self.session = Some(session);
        self
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
