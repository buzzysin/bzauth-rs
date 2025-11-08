use std::sync::Arc;

use axum::handler::Handler;
use axum::routing::{MethodRouter, get, post};
use axum::{Json, Router};
use serde::Serialize;
use serde::ser::SerializeStruct;

use super::routes::{authorise, callback, csrf, logout, refresh, session};
use crate::auth::{Auth, AuthOptions};
use crate::contracts::provide::Provide;

#[derive(Default, Clone)]
pub struct AxumRuntime {
    pub auth: Arc<Auth>,
    pub routes: Router,
}

#[derive(Default)]
pub struct AxumRuntimeOptions {
    pub auth_options: AuthOptions,
}

impl AxumRuntimeOptions {
    /// Create a new Axum runtime options
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_auth_options(self, auth_options: AuthOptions) -> Self {
        Self { auth_options }
    }

    #[must_use]
    pub const fn from_options(auth_options: AuthOptions) -> Self {
        Self { auth_options }
    }
}

impl From<AxumRuntimeOptions> for AxumRuntime {
    fn from(options: AxumRuntimeOptions) -> Self {
        Self::from_options(options)
    }
}

impl AxumRuntime {
    /// Create a new Axum runtime from the given options    
    pub fn from_options(options: AxumRuntimeOptions) -> Self {
        let AxumRuntimeOptions { auth_options } = options;
        let routes = Self::create_router(&auth_options);
        let auth = Arc::new(Auth::from_options(auth_options));

        // Create the runtime
        Self { auth, routes }
    }

    fn create_router(auth_options: &AuthOptions) -> Router {
        let providers = auth_options.providers.clone();
        let providers_handler = move || async move { Json(providers) };

        Router::new()
            // Starts a login flow with a provider
            .route(
                "/login/{provider}",
        {
                    #[cfg(not(debug_assertions))]
                    {
                        post(authorise)
                    }
                    // In debug mode, allow both GET and POST for authorisation
                    #[cfg(debug_assertions)]
                    {
                       any(authorise)
                    }
                }
                ,
            )
            // Where the provider redirects back to (GET or POST)
            .route(
                "/callback/{provider}",
                any(callback),
            )
            // Ask for a csrf token
            .route("/csrf", get(csrf))
            // Get the session for the current user
            .route("/session", get(session))
            // Refresh access token using refresh token
            .route("/refresh/{provider}", post(refresh))
            // Logout endpoint that invalidates the session
            .route("/logout", get(logout))
            // Get a list of providers
            .route("/providers", get(providers_handler))
    }
}

impl Serialize for Box<dyn Provide> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let provider = self.as_ref();
        let provider_name = provider.name();
        let provider_type = provider.provider_type();
        let provider_id = provider.id();

        let mut state = serializer.serialize_struct("Provider", 3)?;
        state.serialize_field("name", &provider_name)?;
        state.serialize_field("type", &provider_type)?;
        state.serialize_field("id", &provider_id)?;
        state.end()
    }
}

fn any<H: Handler<T, S>, T: 'static, S: Clone + Send + Sync + 'static>(f: H) -> MethodRouter<S> {
    post(f.clone()).get(f)
}
