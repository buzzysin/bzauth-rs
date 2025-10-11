use std::collections::HashMap;

use http::header::IntoHeaderName;
use http::{HeaderMap, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;

use super::request::CoreRequest;
use crate::tools::Cookies;

pub trait RequestPayload: Clone + Serialize + Send + Sync {}
impl<T> RequestPayload for T where T: Clone + Serialize + Send + Sync {}

pub trait ResponsePayload: Clone + Serialize + DeserializeOwned + Send + Sync {}
impl<T> ResponsePayload for T where T: Clone + Serialize + DeserializeOwned + Send + Sync {}

#[derive(Debug, Clone)]
pub struct CoreResponse<T = ()>
where
    T: ResponsePayload + 'static,
{
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub cookies: Cookies,
    pub payload: Option<String>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: ResponsePayload> CoreResponse<T> {
    // Constructors/builders

    /// Creates a new `CoreResponse` with default values.
    pub fn new() -> Self {
        Self {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            cookies: Cookies::new(),
            payload: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates a new `CoreResponse` with a 200 OK status.
    pub fn ok() -> Self {
        Self {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            cookies: Cookies::new(),
            payload: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates a new `CoreResponse` that redirects to the specified location. Status code is set to 302 Found.
    /// # Panics
    /// Panics if the location is not a valid URI.
    pub fn redirect(location: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::LOCATION,
            location
                .parse()
                .unwrap_or_else(|_| "http://localhost".parse().unwrap()),
        );

        Self {
            status: StatusCode::FOUND,
            headers,
            cookies: Cookies::new(),
            payload: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates a new `CoreResponse` with a 404 Not Found status.
    pub fn not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            headers: HeaderMap::new(),
            cookies: Cookies::new(),
            payload: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates a new `CoreResponse` with a 500 Internal Server Error status.
    pub fn internal_server_error() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            headers: HeaderMap::new(),
            cookies: Cookies::new(),
            payload: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates a new `CoreResponse` with a specified header. If reassigned, the header will be replaced.
    /// # Panics
    /// Panics if the header value is not valid.
    #[must_use]
    pub fn with_header<H, V>(self, key: H, value: &str) -> Self
    where
        H: IntoHeaderName,
    {
        let mut headers = self.headers.clone();
        headers.insert(key, value.parse().unwrap());

        Self { headers, ..self }
    }

    /// Creates a new `CoreResponse` with multiple headers. If reassigned, the headers will be replaced.
    /// # Panics
    /// Panics if any header value is not valid.
    #[must_use]
    pub fn with_headers<H>(self, headers: HashMap<H, String>) -> Self
    where
        H: IntoHeaderName,
    {
        let mut new_headers = self.headers.clone();
        for (key, value) in headers {
            new_headers.insert(key, value.parse().unwrap());
        }

        Self {
            headers: new_headers,
            ..self
        }
    }

    /// Append a redirect header to the response
    /// # Panics
    /// Panics if the location is not a valid URI.
    #[must_use]
    pub fn with_redirect(self, location: &str) -> Self {
        let mut headers = self.headers.clone();
        headers.insert(http::header::LOCATION, location.parse().unwrap());

        Self {
            status: StatusCode::FOUND,
            headers,
            ..self
        }
    }

    /// Creates a new `CoreResponse` with a specified cookie. If reassigned, the cookie will be replaced.
    /// # Panics
    /// Panics if the cookie value is not valid.
    #[must_use]
    pub fn with_cookie(self, name: String, value: String) -> Self {
        let mut cookies = self.cookies.clone();
        cookies.set(name, value);

        Self { cookies, ..self }
    }

    /// Creates a new `CoreResponse` with multiple cookies. If reassigned, the cookies will be replaced.
    /// # Panics
    /// Panics if any cookie value is not valid.
    #[must_use]
    pub fn with_cookies(self, cookies: Cookies) -> Self {
        let mut new_cookies = self.cookies.clone();
        new_cookies.extend(cookies);

        Self {
            cookies: new_cookies,
            ..self
        }
    }

    /// Creates a new `CoreResponse` with a specified status code.
    /// # Panics
    /// Panics if the status code is not valid.
    #[must_use]
    pub fn with_status(self, status: StatusCode) -> Self {
        Self { status, ..self }
    }

    /// Creates a new `CoreResponse` from an existing `CoreRequest`, copying headers and cookies.
    /// # Panics
    /// Panics if the request payload cannot be serialized to JSON.
    #[must_use]
    pub fn from_request<U>(request: &CoreRequest<U>) -> Self
    where
        U: RequestPayload,
    {
        // Copy headers and cookies from the request
        Self {
            status: StatusCode::OK,
            headers: request.headers().clone(),
            cookies: request.cookies().clone(),
            payload: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Merges headers and cookies from an existing `CoreRequest` into the `CoreResponse`.
    /// # Panics
    /// Panics if the request payload cannot be serialized to JSON.
    #[must_use]
    pub fn with_session(self, request: &CoreRequest<T>) -> Self
    where
        T: RequestPayload,
    {
        // Merge the cookies from the request into the response
        let mut cookies = self.cookies.clone();
        cookies.extend(request.cookies().clone());

        let mut headers = self.headers.clone();
        headers.extend(request.headers().clone());

        Self {
            status: self.status,
            headers,
            cookies,
            payload: self.payload,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates a new `CoreResponse` with a specified payload. Sets the `Content-Type` header to `application/json`.
    /// # Panics
    /// Panics if the payload cannot be serialized to JSON.
    #[must_use]
    pub fn with_payload<U>(self, payload: &U) -> CoreResponse<U>
    where
        U: ResponsePayload,
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

    pub const fn status(&self) -> StatusCode {
        self.status
    }

    pub const fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    pub const fn cookies(&self) -> &Cookies {
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
        Self::new()
    }
}
