use axum::extract::Request;

use crate::runtimes::axum::extractors::auth::ExtractAuth;
use crate::tools::request::CoreRequest;
use crate::tools::response::CoreResponse;
use crate::tools::{self, CoreError, SessionResponse, TryFromAsync};

#[axum::debug_handler]
pub async fn session(
    ExtractAuth(auth): ExtractAuth,
    request: Request,
) -> Result<CoreResponse<SessionResponse>, CoreError> {
    // Pass to internal handler
    let core_request = CoreRequest::try_from_async(request).await?.with_auth(auth);
    tools::session(&core_request).await
}
