use axum::extract::Request;

use crate::runtimes::axum::extractors::auth::ExtractAuth;
use crate::tools;
use crate::tools::request::CoreRequest;
use crate::tools::response::CoreResponse;
use crate::tools::try_async::TryFromAsync;
use crate::tools::{AuthoriseResponse, CoreError};

#[axum::debug_handler]
pub async fn authorise(
    ExtractAuth(auth): ExtractAuth,
    request: Request,
) -> Result<CoreResponse<AuthoriseResponse>, CoreError> {
    // Pass to internal handler
    let core_request = CoreRequest::try_from_async(request).await?.with_auth(auth);
    tools::authorise(core_request).await
}
