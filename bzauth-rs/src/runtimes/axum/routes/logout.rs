use axum::extract::Request;

use crate::runtimes::axum::extractors::auth::ExtractAuth;
use crate::tools::request::CoreRequest;
use crate::tools::response::CoreResponse;
use crate::tools::{self, CoreError, LogoutResponse, TryFromAsync};

#[axum::debug_handler]
pub async fn logout(
    ExtractAuth(auth): ExtractAuth,
    request: Request,
) -> Result<CoreResponse<LogoutResponse>, CoreError> {
    // Pass to internal handler
    let core_request = CoreRequest::try_from_async(request).await?.with_auth(auth);
    tools::logout(&core_request).await
}
