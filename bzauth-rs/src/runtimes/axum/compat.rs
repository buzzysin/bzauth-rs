/// Compatibility layer for the Axum runtime with the `CoreRequest` and `CoreResponse` types.
/// This module provides the necessary conversions and implementations to allow
/// using Axum's request and response types with the `CoreRequest` and `CoreResponse` types.
use axum::{Json, RequestExt, extract::Request, response::IntoResponse};

use crate::tools::CoreError;
use crate::tools::request::CoreRequest;
use crate::tools::response::{CoreResponse, RequestPayload, ResponsePayload};
use crate::tools::try_async::TryFromAsync;

impl<T: RequestPayload> TryFromAsync<Request> for CoreRequest<T> {
    type Error = CoreError;

    async fn try_from_async(request: Request) -> Result<Self, Self::Error> {
        tracing::debug!("[compat:axum] Converting Axum request to CoreRequest");

        let path = request.uri().path().to_string();
        tracing::debug!("[compat:axum] Request URI: {}", request.uri());

        let method = request.method().to_string();
        tracing::debug!("[compat:axum] Request method: {}", request.method());

        let uri = request.uri().clone();
        tracing::debug!("[compat:axum] Request URI: {}", uri);

        let headers = request.headers().clone();
        tracing::debug!("[compat:axum] Request headers: {:?}", headers);

        let cookies = request
            .headers()
            .get(axum::http::header::COOKIE)
            .cloned()
            .unwrap_or_else(|| {
                tracing::debug!("[compat:axum] No cookies header found in request");
                axum::http::HeaderValue::from_static("")
            })
            .to_str()
            .ok()
            .unwrap_or_else(|| {
                tracing::debug!("[compat:axum] No cookies found in request headers");
                ""
            })
            .to_string()
            .parse()
            .unwrap_or_default();
        tracing::debug!("[compat:axum] Request cookies: {:?}", cookies);

        // The auth cannot be read directly from the request, it must be passed in
        let auth = None;

        // The body is not directly accessible in the request, so we will set it to None
        let body = request
            .extract::<String, _>()
            .await
            .map_err(|_| CoreError::new().with_message("Failed to extract body"));
        tracing::debug!("[compat:axum] Request body: {:?}", body);

        // Create the CoreRequest
        Ok(Self::new_unchecked(
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
    T: ResponsePayload,
{
    #[allow(clippy::cognitive_complexity)]
    fn into_response(self) -> axum::response::Response {
        let mut response = axum::response::Response::default();

        // Set the body
        tracing::debug!("[compat:axum] Setting response body: {:?}", self.payload);
        if let Some(body) = self.payload {
            *response.body_mut() = axum::body::Body::from(
                serde_json::to_string(&body)
                    .unwrap_or_else(|_| "Failed to serialize response".to_string()),
            );
        }

        // Set the status code
        tracing::debug!("[compat:axum] Setting status code: {}", self.status);
        *response.status_mut() = axum::http::StatusCode::from_u16(self.status.into())
            .unwrap_or(axum::http::StatusCode::OK);

        // Set the headers
        tracing::debug!("[compat:axum] Setting headers: {:?}", self.headers);
        for (key, value) in self.headers {
            if let Some(key) = key {
                response.headers_mut().insert(key, value);
            }
        }

        // Set the cookies
        tracing::debug!("[compat:axum] Setting cookies: {:?}", self.cookies);
        for (_, cookie) in self.cookies.iter() {
            response.headers_mut().append(
                axum::http::header::SET_COOKIE,
                cookie
                    .to_string()
                    .parse()
                    .expect("Invalid cookie header format"),
            );
        }

        tracing::debug!("[compat:axum] Final response: {:?}", response);

        response
    }
}

impl IntoResponse for CoreError {
    fn into_response(self) -> axum::response::Response {
        (http::StatusCode::from_u16(self.status).unwrap(), Json(self)).into_response()
    }
}
