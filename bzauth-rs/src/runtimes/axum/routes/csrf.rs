use axum::extract::Request;

use crate::{
    runtimes::axum::extractors::auth::ExtractAuth,
    tools::{self, CoreError, TryFromAsync, request::CoreRequest, response::CoreResponse},
};

#[axum::debug_handler]
pub async fn csrf(
    ExtractAuth(auth): ExtractAuth,
    request: Request,
) -> Result<CoreResponse, CoreError> {
    // Pass to internal handler
    let core_request = CoreRequest::try_from_async(request).await?.with_auth(auth);
    tools::csrf(core_request).await
}
