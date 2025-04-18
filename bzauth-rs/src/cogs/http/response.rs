use std::collections::HashMap;

use http::{HeaderMap, StatusCode, header::IntoHeaderName};
use serde::Serialize;

use super::request::CoreRequest;
use crate::cogs::Cookies;

#[derive(Debug, Clone)]
pub struct CoreResponse<Payload: Serialize = String> {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub cookies: Cookies,
    pub body: Option<Payload>,
}

impl<Payload: Serialize> CoreResponse<Payload> {
    pub fn new() -> Self {
        CoreResponse {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            cookies: Cookies::new(),
            body: None,
        }
    }

    pub fn ok() -> Self {
        CoreResponse {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            cookies: Cookies::new(),
            body: None,
        }
    }

    pub fn redirect(location: String) -> Self
    where
        Payload: Default,
    {
        CoreResponse {
            status: StatusCode::FOUND,
            headers: {
                let mut headers = HeaderMap::new();
                headers.insert(
                    http::header::LOCATION,
                    location
                        .parse()
                        .unwrap_or_else(|_| "http://localhost".parse().unwrap()),
                );
                headers
            },
            cookies: Cookies::new(),
            body: None,
        }
    }

    pub fn not_found() -> Self
    where
        Payload: Default,
    {
        CoreResponse {
            status: StatusCode::NOT_FOUND,
            headers: HeaderMap::new(),
            cookies: Cookies::new(),
            body: None,
        }
    }

    pub fn internal_server_error() -> Self
    where
        Payload: Default,
    {
        CoreResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            headers: HeaderMap::new(),
            cookies: Cookies::new(),
            body: None,
        }
    }

    pub fn with_header<H, V>(mut self, key: H, value: String) -> Self
    where
        H: IntoHeaderName,
    {
        self.headers.insert(key, value.parse().unwrap());
        self
    }

    pub fn with_headers<H>(mut self, headers: HashMap<H, String>) -> Self
    where
        H: IntoHeaderName,
    {
        for (key, value) in headers {
            self.headers.insert(key, value.parse().unwrap());
        }
        self
    }

    pub fn with_cookie(mut self, name: String, value: String) -> Self {
        self.cookies.set(name, value);
        self
    }

    pub fn with_cookies(mut self, cookies: Cookies) -> Self {
        self.cookies.extend(cookies);
        self
    }

    pub fn with_body(mut self, body: Payload) -> Self {
        self.body = Some(body);
        self
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn from_request(request: CoreRequest) -> Self {
        // Copy headers and cookies from the request
        CoreResponse {
            status: StatusCode::OK,
            headers: request.headers.clone(),
            cookies: request.cookies.clone(),
            body: None,
        }
    }
}
