use axum::extract::Request;

use crate::{
    cogs::{self, CoreError, request::CoreRequest, response::CoreResponse},
    runtimes::axum::extractors::auth::ExtractAuth,
};

#[axum::debug_handler]
pub async fn csrf(
    ExtractAuth(auth): ExtractAuth,
    request: Request,
) -> Result<CoreResponse, CoreError> {
    // Pass to internal handler
    let core_request = CoreRequest::from(request).with_auth(auth);
    cogs::csrf(core_request).await
}
