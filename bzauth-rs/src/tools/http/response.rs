use std::collections::HashMap;

use http::{HeaderMap, StatusCode, header::IntoHeaderName};
use serde::Serialize;

use super::request::CoreRequest;
use crate::tools::Cookies;

pub trait ValidPayload: Clone + Serialize {}
impl<T> ValidPayload for T where T: Clone + Serialize {}

#[derive(Debug, Clone)]
pub struct CoreResponse<Payload = String> {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub cookies: Cookies,
    pub payload: Option<Payload>,
}

impl<Payload> CoreResponse<Payload> {
    // Constructors/builders

    /// Creates a new CoreResponse with default values.
    pub fn new() -> Self {
        CoreResponse {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            cookies: Cookies::new(),
            payload: None,
        }
    }

    /// Creates a new CoreResponse with a 200 OK status.
    pub fn ok() -> Self {
        CoreResponse {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            cookies: Cookies::new(),
            payload: None,
        }
    }

    /// Creates a new CoreResponse that redirects to the specified location. Status code is set to 302 Found.
    pub fn redirect(location: String) -> Self
    where
        Payload: Default,
    {
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::LOCATION,
            location
                .parse()
                .unwrap_or_else(|_| "http://localhost".parse().unwrap()),
        );

        CoreResponse {
            status: StatusCode::FOUND,
            headers,
            cookies: Cookies::new(),
            payload: None,
        }
    }

    /// Creates a new CoreResponse with a 404 Not Found status.
    pub fn not_found() -> Self
    where
        Payload: Default,
    {
        CoreResponse {
            status: StatusCode::NOT_FOUND,
            headers: HeaderMap::new(),
            cookies: Cookies::new(),
            payload: None,
        }
    }

    /// Creates a new CoreResponse with a 500 Internal Server Error status.
    pub fn internal_server_error() -> Self
    where
        Payload: Default,
    {
        CoreResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            headers: HeaderMap::new(),
            cookies: Cookies::new(),
            payload: None,
        }
    }

    /// Creates a new CoreResponse with a specified header. If reassigned, the header will be replaced.
    pub fn with_header<H, V>(self, key: H, value: String) -> Self
    where
        H: IntoHeaderName,
    {
        let mut headers = self.headers.clone();
        headers.insert(key, value.parse().unwrap());

        CoreResponse { headers, ..self }
    }

    /// Creates a new CoreResponse with multiple headers. If reassigned, the headers will be replaced.
    pub fn with_headers<H>(self, headers: HashMap<H, String>) -> Self
    where
        H: IntoHeaderName,
    {
        let mut new_headers = self.headers.clone();
        for (key, value) in headers {
            new_headers.insert(key, value.parse().unwrap());
        }

        CoreResponse {
            headers: new_headers,
            ..self
        }
    }

    // Append a redirect header to the response
    pub fn with_redirect(self, location: String) -> Self {
        let mut headers = self.headers.clone();
        headers.insert(
            http::header::LOCATION,
            location
                .parse()
                .unwrap_or_else(|_| "http://localhost".parse().unwrap()),
        );

        CoreResponse {
            status: StatusCode::FOUND,
            headers,
            ..self
        }
    }

    pub fn with_cookie(self, name: String, value: String) -> Self {
        let mut cookies = self.cookies.clone();
        cookies.set(name, value);

        CoreResponse { cookies, ..self }
    }

    pub fn with_cookies(self, cookies: Cookies) -> Self {
        let mut new_cookies = self.cookies.clone();
        new_cookies.extend(cookies);

        CoreResponse {
            cookies: new_cookies,
            ..self
        }
    }

    pub fn with_status(self, status: StatusCode) -> Self {
        CoreResponse { status, ..self }
    }

    pub fn from_request(request: &CoreRequest) -> Self {
        // Copy headers and cookies from the request
        CoreResponse {
            status: StatusCode::OK,
            headers: request.headers().clone(),
            cookies: request.cookies().clone(),
            payload: None,
        }
    }

    pub fn with_session(self, request: &CoreRequest) -> Self {
        // Merge the cookies from the request into the response
        let mut cookies = self.cookies.clone();
        cookies.extend(request.cookies().clone());

        let mut headers = self.headers.clone();
        headers.extend(request.headers().clone());

        CoreResponse {
            status: self.status,
            headers,
            cookies,
            payload: self.payload,
        }
    }

    pub fn with_payload<T>(self, payload: T) -> CoreResponse
    where
        T: ValidPayload,
    {
        let mut headers = self.headers.clone();
        headers.insert(
            http::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        CoreResponse {
            status: self.status,
            headers,
            cookies: self.cookies,
            payload: Some(serde_json::to_string(&payload).ok().unwrap_or_else(|| {
                tracing::error!("Failed to serialize payload");
                String::new()
            })),
        }
    }

    // Getters

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    pub fn cookies(&self) -> &Cookies {
        &self.cookies
    }

    pub fn cookies_mut(&mut self) -> &mut Cookies {
        &mut self.cookies
    }

    pub fn body(&self) -> Option<&Payload> {
        self.payload.as_ref()
    }

    pub fn body_mut(&mut self) -> &mut Option<Payload> {
        &mut self.payload
    }
}

impl Default for CoreResponse<String> {
    fn default() -> Self {
        CoreResponse::new()
    }
}
