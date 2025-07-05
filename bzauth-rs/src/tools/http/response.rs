use std::collections::HashMap;

use http::header::IntoHeaderName;
use http::{HeaderMap, StatusCode};
use serde::Serialize;

use super::request::CoreRequest;
use crate::tools::Cookies;

pub trait RequestPayload: Clone + Serialize {}
impl<T> RequestPayload for T where T: Clone + Serialize {}

pub trait ResponsePayload: Clone + serde::de::DeserializeOwned {}
impl<T> ResponsePayload for T where T: Clone + serde::de::DeserializeOwned {}

#[derive(Debug, Clone)]
pub struct CoreResponse<T = ()> {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub cookies: Cookies,
    pub payload: Option<String>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> CoreResponse<T> {
    // Constructors/builders

    /// Creates a new CoreResponse with default values.
    pub fn new() -> Self {
        CoreResponse {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            cookies: Cookies::new(),
            payload: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates a new CoreResponse with a 200 OK status.
    pub fn ok() -> Self {
        CoreResponse {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            cookies: Cookies::new(),
            payload: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates a new CoreResponse that redirects to the specified location. Status code is set to 302 Found.
    pub fn redirect(location: String) -> Self {
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
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates a new CoreResponse with a 404 Not Found status.
    pub fn not_found() -> Self {
        CoreResponse {
            status: StatusCode::NOT_FOUND,
            headers: HeaderMap::new(),
            cookies: Cookies::new(),
            payload: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates a new CoreResponse with a 500 Internal Server Error status.
    pub fn internal_server_error() -> Self {
        CoreResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            headers: HeaderMap::new(),
            cookies: Cookies::new(),
            payload: None,
            _phantom: std::marker::PhantomData,
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
        headers.insert(http::header::LOCATION, location.parse().unwrap());

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

    pub fn from_request<U>(request: &CoreRequest<U>) -> Self
    where
        U: RequestPayload,
    {
        // Copy headers and cookies from the request
        CoreResponse {
            status: StatusCode::OK,
            headers: request.headers().clone(),
            cookies: request.cookies().clone(),
            payload: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_session(self, request: &CoreRequest<T>) -> Self {
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
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_payload<U>(self, payload: U) -> CoreResponse<U>
    where
        U: RequestPayload,
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
            _phantom: std::marker::PhantomData,
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

    pub fn body(&self) -> Option<T>
    where
        T: RequestPayload + serde::de::DeserializeOwned,
    {
        self.payload
            .as_ref()
            .and_then(|body| serde_json::from_str(body).ok())
            .or_else(|| {
                tracing::warn!("Response body is not set or cannot be deserialized");
                None
            })
    }

    pub fn set_body<U>(&mut self, body: U)
    where
        U: RequestPayload,
    {
        self.payload = Some(serde_json::to_string(&body).unwrap_or_else(|_| {
            tracing::error!("Failed to serialize response body");
            String::new()
        }));
    }
}

impl Default for CoreResponse<String> {
    fn default() -> Self {
        CoreResponse::new()
    }
}
