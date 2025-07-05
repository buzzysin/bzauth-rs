use axum::extract::Request;

use crate::runtimes::axum::extractors::auth::ExtractAuth;
use crate::tools::request::CoreRequest;
use crate::tools::response::CoreResponse;
use crate::tools::{self, CoreError, TryFromAsync};

#[axum::debug_handler]
pub async fn csrf(
    ExtractAuth(auth): ExtractAuth,
    request: Request,
) -> Result<CoreResponse, CoreError> {
    // Pass to internal handler
    let core_request = CoreRequest::try_from_async(request).await?.with_auth(auth);
    tools::csrf(core_request).await
}
