use axum::RequestPartsExt;
use axum::extract::{FromRequestParts, Path};
use axum::http::request::Parts;
use axum::response::IntoResponse;
use serde::Serialize;

use super::auth::ExtractAuth;
use crate::contracts::provide::Provide;

pub struct ExtractProvider(pub Box<dyn Provide>);

#[derive(Debug, Serialize)]
pub enum ExtractProviderError {
    MissingAuth(String),
    MissingProvider(String),
}

impl std::fmt::Display for ExtractProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtractProviderError::MissingAuth(err) => write!(f, "{}", err),
            ExtractProviderError::MissingProvider(err) => write!(f, "{}", err),
        }
    }
}
impl std::error::Error for ExtractProviderError {}

impl IntoResponse for ExtractProviderError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match self {
            ExtractProviderError::MissingAuth(err) => axum::http::Response::builder()
                .status(401)
                .body(axum::body::Body::from(err))
                .unwrap(),

            ExtractProviderError::MissingProvider(err) => axum::http::Response::builder()
                .status(404)
                .body(axum::body::Body::from(err))
                .unwrap(),
        }
    }
}

impl<S> FromRequestParts<S> for ExtractProvider
where
    S: Send + Sync,
{
    type Rejection = ExtractProviderError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the auth object from the request parts
        let ExtractAuth(auth) = ExtractAuth::from_request_parts(parts, state)
            .await
            .map_err(|err| {
                ExtractProviderError::MissingAuth(format!("Failed to extract auth: {}", err))
            })?;

        // Get the list of providers from the auth object
        let providers = &auth.options.providers;

        // Get the matched path from the request
        let Path(provider_id) = parts.extract::<Path<String>>().await.map_err(|err| {
            ExtractProviderError::MissingAuth(format!("No provider found in path: {}", err))
        })?;

        // Find the provider in the list of providers
        let provider = providers
            .iter()
            .find(|p| p.id() == provider_id)
            .ok_or_else(|| {
                ExtractProviderError::MissingProvider(format!("Provider {} not found", provider_id))
            })?;

        Ok(Self(provider.clone()))
    }
}
