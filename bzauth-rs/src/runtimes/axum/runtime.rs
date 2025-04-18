use std::sync::Arc;

use axum::{
    Json, Router,
    routing::{get, post},
};
use serde::{Serialize, ser::SerializeStruct};

use super::routes::{authorise, callback, csrf};
use crate::auth::Auth;
use crate::{auth::AuthOptions, contracts::provide::Provide};
pub struct AxumRuntime {
    pub auth: Arc<Auth>,
    pub routes: Router,
}

pub struct AxumRuntimeOptions {
    pub auth_options: AuthOptions,
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

impl AxumRuntime {
    /// Create a new Axum runtime
    pub fn from_options(options: AxumRuntimeOptions) -> Self {
        let AxumRuntimeOptions { auth_options } = options;
        let routes = AxumRuntime::create_router(&auth_options);
        let auth = Arc::new(Auth::from_options(auth_options));

        // Create the runtime
        let runtime = AxumRuntime { auth, routes };
        runtime
    }

    pub fn create_router(auth_options: &AuthOptions) -> Router {
        let providers = auth_options.providers.clone();
        let providers_handler = move || async move { Json(providers) };

        Router::new()
            // Starts a login flow with a provider
            .route(
                "/login/{provider}",
                post(authorise),
            )
            // Where the provider redirects back to (GET or POST)
            .route(
                "/callback/{provider}",
                get(callback)
                    .post(callback),
            )
            // Ask for a csrf token
            .route("/csrf", get(csrf))
            // Get the session for the current user
            .route("/session", get(|| async { "session endpoint" }))
            // Logout endpoint that invalidates the session
            .route("/logout", get(|| async { "Logout endpoint" }))
            // Get a list of providers
            .route("/providers", get(providers_handler))
    }
}
