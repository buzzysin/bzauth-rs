use axum::extract::Request;

use crate::tools::{
    AuthoriseResponse, CoreError, request::CoreRequest, response::CoreResponse,
    try_async::TryFromAsync,
};
use crate::{runtimes::axum::extractors::auth::ExtractAuth, tools};

pub async fn authorise(
    ExtractAuth(auth): ExtractAuth,
    request: Request,
) -> Result<CoreResponse<AuthoriseResponse>, CoreError> {
    // Pass to internal handler
    let core_request = CoreRequest::try_from_async(request).await?.with_auth(auth);
    tools::authorise(core_request).await
}
