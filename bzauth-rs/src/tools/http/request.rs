use std::collections::HashMap;
use std::sync::Arc;

use http::{HeaderMap, HeaderName, Uri};
use serde::de::DeserializeOwned;

use super::cookie::Cookies;
use crate::auth::Auth;
use crate::tools::response::RequestPayload;

#[derive(Clone)]
pub struct CoreRequest<T = ()> {
    path: String,
    method: String,
    uri: Uri,
    headers: HeaderMap,
    cookies: Cookies,
    body: Option<String>,
    auth: Option<Arc<Auth>>,
    _phantom: std::marker::PhantomData<T>,
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
    ) -> Self
    where
        T: RequestPayload,
    {
        Self {
            path,
            method,
            uri,
            headers,
            cookies,
            body: body.map(|b| serde_json::to_string(&b).unwrap()),
            auth,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn new_unchecked(
        path: String,
        method: String,
        uri: Uri,
        headers: HeaderMap,
        cookies: Cookies,
        body: Option<String>,
        auth: Option<Arc<Auth>>,
    ) -> Self {
        Self {
            path,
            method,
            uri,
            headers,
            cookies,
            body, // careful - we didn't check if this is a valid RequestPayload
            auth,
            _phantom: std::marker::PhantomData,
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

    pub fn body(&self) -> Option<T>
    where
        T: RequestPayload + DeserializeOwned,
    {
        self.body
            .as_ref()
            .and_then(|body| serde_json::from_str(body).ok())
    }

    pub fn map_body<F, U>(&self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
        T: RequestPayload + DeserializeOwned,
    {
        self.body().map(f)
    }

    pub fn auth(&self) -> Option<&Arc<Auth>> {
        self.auth.as_ref()
    }
}
