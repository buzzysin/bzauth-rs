use std::{collections::HashMap, sync::Arc};

use http::Uri;
use http::{HeaderMap, HeaderName};

use super::cookie::Cookies;
use crate::auth::Auth;

#[derive(Clone)]
pub struct CoreRequest<Body = String> {
    path: String,
    method: String,
    uri: Uri,
    headers: HeaderMap,
    cookies: Cookies,
    body: Option<Body>,
    auth: Option<Arc<Auth>>,
}

impl<T> CoreRequest<T> {
    pub fn new(
        path: String,
        method: String,
        uri: Uri,
        headers: HeaderMap,
        cookies: Cookies,
        body: Option<T>,
        auth: Option<Arc<Auth>>,
    ) -> Self {
        Self {
            path,
            method,
            uri,
            headers,
            cookies,
            body,
            auth,
        }
    }

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

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn cookies(&self) -> &Cookies {
        &self.cookies
    }

    pub fn body(&self) -> Option<&T> {
        self.body.as_ref()
    }

    pub fn auth(&self) -> Option<&Arc<Auth>> {
        self.auth.as_ref()
    }
}
