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

pub struct AuthOptions {
    pub providers: Vec<Box<dyn Provide>>,
    pub adaptor: Option<Box<dyn Adapt>>,
    pub callbacks: Option<AuthCallbackOptions>,
    pub session: Option<AuthSessionOptions>,
}
pub struct Auth {
    pub options: AuthOptions,
}

impl Auth {
    pub fn from_options(options: AuthOptions) -> Self {
        Self { options }
    }
}
