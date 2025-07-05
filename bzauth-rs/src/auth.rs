use std::ops::Deref;
use std::sync::Arc;

use url::Url;

use crate::awaitable;
use crate::contracts::account::Account;
use crate::contracts::adapt::Adapt;
use crate::contracts::profile::Profile;
use crate::contracts::provide::Provide;
use crate::contracts::user::User;
use crate::tools::awaitable::Awaitable;

#[derive(Debug, Clone)]
pub struct SignInOptions {
    pub user: Option<User>,
    pub account: Option<Account>,
    pub profile: Option<Profile>,
}

#[derive(Debug, Clone)]
pub enum SignInResult {
    Success,
    Redirect(String),
    Error(String),
}

pub type SignInCallback = Arc<dyn Fn(SignInOptions) -> Awaitable<SignInResult> + Send + Sync>;

#[derive(Clone)]
pub struct RedirectCallback(Arc<dyn Fn(String, String) -> Awaitable<String> + Send + Sync>);

impl RedirectCallback {
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(String, String) -> Awaitable<String> + Send + Sync + 'static,
    {
        Self(Arc::new(callback))
    }
}

impl Deref for RedirectCallback {
    type Target = Arc<dyn Fn(String, String) -> Awaitable<String> + Send + Sync>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for RedirectCallback {
    /// The default redirect only allows same-origin redirects. The user can configure this to their liking.
    fn default() -> Self {
        Self(Arc::new(|url, base_url| {
            // Extract the url data
            let url_obj = Url::parse(&base_url).ok();
            let origin = url_obj.as_ref().map(|u| u.origin().ascii_serialization());

            // If the url is same-origin, return it
            if let Some(origin) = origin {
                if url.starts_with(&origin) {
                    return awaitable!(url);
                }
            }

            // Otherwise, go back to the origin
            awaitable!(base_url)
        }))
    }
}

#[derive(Clone, Default)]
pub struct AuthCallbackOptions {
    pub sign_in: Option<SignInCallback>,
    pub redirect: RedirectCallback,
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
    pub fn with_callback(self, callback: SignInCallback) -> Self {
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

    pub fn adaptor(&self) -> Option<&dyn Adapt> {
        self.options.adaptor.as_ref().map(|a| a.as_ref())
    }
}
