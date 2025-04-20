use std::{collections::HashMap, sync::Arc};

use http::Uri;
use http::{HeaderMap, HeaderName};

use crate::{auth::Auth, cogs::Cookies};

#[derive(Clone)]
pub struct CoreRequest<Body = String> {
    pub path: String,
    pub method: String,
    pub uri: Uri,
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

    pub fn query(&self) -> HashMap<String, String> {
        let mut query = HashMap::new();
        if let Some(query_string) = self.uri.query() {
            for (key, value) in url::form_urlencoded::parse(query_string.as_bytes()) {
                query.insert(key.to_string(), value.to_string());
            }
        }
        query
    }

    pub fn header(&self, key: HeaderName) -> Option<String> {
        self.headers
            .get(key)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
    }
}
