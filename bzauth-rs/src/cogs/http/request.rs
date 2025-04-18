use std::sync::Arc;

use http::HeaderMap;

use crate::{auth::Auth, cogs::Cookies};

#[derive(Clone)]
pub struct CoreRequest<Body = String> {
    pub path: String,
    pub method: String,
    pub query: String,
    pub headers: HeaderMap,
    pub cookies: Cookies,
    pub body: Option<Body>,
    pub auth: Option<Arc<Auth>>,
}

impl<T> CoreRequest<T> {
    pub fn with_auth(self, auth: Arc<Auth>) -> Self {
        Self {
            auth: Some(auth),
            ..self
        }
    }
}
