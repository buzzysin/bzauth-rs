use std::sync::Arc;

use axum::{Extension, Json, extract::Request, response::IntoResponse};
use serde::Serialize;

use crate::{
    auth::Auth,
    cogs::{Cookies, CoreError, request::CoreRequest, response::CoreResponse},
};

pub mod extractors;
pub mod routes;
pub mod runtime;

impl<Payload> From<Request<Payload>> for CoreRequest<Payload> {
    fn from(request: Request<Payload>) -> Self {
        let auth = request
            .extensions()
            .get::<Extension<Arc<Auth>>>()
            .cloned()
            .map(|ext| ext.0.clone());

        let path = request.uri().path().to_string();
        let method = request.method().to_string();
        let uri = request.uri().clone();
        let headers = request.headers().clone();
        let cookies: Cookies = request
            .headers()
            .get(axum::http::header::COOKIE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string()
            .parse()
            .unwrap_or_default();

        // Request consumed after this point
        let body = Some(request.into_body());

        // Build the request#
        CoreRequest {
            path,
            method,
            uri,
            headers,
            cookies, // Placeholder for cookies, you can implement this as needed
            body,
            auth,
        }
    }
}

impl<T: Serialize> IntoResponse for CoreResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let mut response = axum::response::Response::new("".into());

        // Set the body
        if let Some(body) = self.body {
            *response.body_mut() = Json::<T>(body).into_response().into_body();
        }

        // Set the status code
        *response.status_mut() = axum::http::StatusCode::from_u16(self.status.into())
            .unwrap_or(axum::http::StatusCode::OK);

        // Set the headers
        for (key, value) in self.headers {
            if let Some(key) = key {
                response.headers_mut().insert(key, value);
            }
        }

        // Set the cookies
        for (_, value) in self.cookies.iter() {
            response.headers_mut().append(
                axum::http::header::SET_COOKIE,
                value.unparse().parse().unwrap(),
            );
        }

        response
    }
}

impl IntoResponse for CoreError {
    fn into_response(self) -> axum::response::Response {
        (http::StatusCode::from_u16(self.status).unwrap(), Json(self)).into_response()
    }
}
