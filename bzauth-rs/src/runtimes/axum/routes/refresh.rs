use axum::extract::Request;

use crate::runtimes::axum::extractors::auth::ExtractAuth;
use crate::tools::request::CoreRequest;
use crate::tools::response::CoreResponse;
use crate::tools::{self, CoreError, RefreshResponse, TryFromAsync};

#[axum::debug_handler]
pub async fn refresh(
    ExtractAuth(auth): ExtractAuth,
    request: Request,
) -> Result<CoreResponse<RefreshResponse>, CoreError> {
    // Pass to internal handler
    let core_request = CoreRequest::try_from_async(request).await?.with_auth(auth);
    tools::refresh(&core_request).await
}
