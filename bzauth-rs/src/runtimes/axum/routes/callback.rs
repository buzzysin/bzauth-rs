use axum::extract::Request;

use crate::{
    cogs::{self, CoreError, request::CoreRequest, response::CoreResponse},
    runtimes::axum::extractors::auth::ExtractAuth,
};

#[axum::debug_handler]
pub async fn callback(
    ExtractAuth(auth): ExtractAuth,
    request: Request,
) -> Result<CoreResponse, CoreError> {
    // Pass to internal handler
    let core_request = CoreRequest::from(request).with_auth(auth);
    cogs::callback(core_request).await
}
