/// Compatibility layer for the Axum runtime with the CoreRequest and CoreResponse types.
/// This module provides the necessary conversions and implementations to allow
/// using Axum's request and response types with the CoreRequest and CoreResponse types.
use axum::{Json, RequestExt, extract::Request, response::IntoResponse};
use serde::Serialize;

use crate::tools::CoreError;
use crate::tools::request::CoreRequest;
use crate::tools::response::{CoreResponse, RequestPayload};
use crate::tools::try_async::TryFromAsync;

impl<T: RequestPayload> TryFromAsync<Request> for CoreRequest<T> {
    type Error = CoreError;

    async fn try_from_async(request: Request) -> Result<Self, Self::Error> {
        let path = request.uri().path().to_string();
        let method = request.method().to_string();
        let uri = request.uri().clone();
        let headers = request.headers().clone();
        let cookies = request
            .headers()
            .get(axum::http::header::COOKIE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string()
            .parse()
            .unwrap_or_default();

        // The auth cannot be read directly from the request, it must be passed in
        let auth = None;

        // The body is not directly accessible in the request, so we will set it to None
        let body = request
            .extract::<String, _>()
            .await
            .map_err(|_| CoreError::new().with_message("Failed to extract body"));

        // Create the CoreRequest
        Ok(CoreRequest::new_unchecked(
            path,
            method,
            uri,
            headers,
            cookies,
            body.ok(),
            auth,
        ))
    }
}

impl<T> IntoResponse for CoreResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let mut response = axum::response::Response::default();

        // Set the body
        if let Some(body) = self.payload {
            *response.body_mut() = axum::body::Body::from(
                serde_json::to_string(&body)
                    .unwrap_or_else(|_| "Failed to serialize response".to_string()),
            );
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
