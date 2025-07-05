use std::sync::Arc;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum::{Extension, RequestPartsExt};
use serde::Serialize;

use crate::auth::Auth;

pub struct ExtractAuth(pub(crate) Arc<Auth>);

#[derive(Debug, Serialize)]
pub enum ExtractAuthError {
    MissingAuth(String),
}

impl std::fmt::Display for ExtractAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtractAuthError::MissingAuth(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for ExtractAuthError {}

impl IntoResponse for ExtractAuthError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match self {
            ExtractAuthError::MissingAuth(err) => axum::http::Response::builder()
                .status(401)
                .body(axum::body::Body::from(err))
                .unwrap(),
        }
    }
}

impl<S> FromRequestParts<S> for ExtractAuth
where
    S: Send + Sync,
{
    type Rejection = ExtractAuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the auth object from the request parts
        let auth = parts
            .extract::<Extension<Arc<Auth>>>()
            .await
            .map_err(|err| {
                ExtractAuthError::MissingAuth(format!("Failed to extract auth: {}", err))
            })
            .map(|auth| auth.0.clone())?;

        Ok(Self(auth))
    }
}
