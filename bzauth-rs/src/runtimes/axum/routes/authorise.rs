use axum::extract::Request;

use crate::cogs::{CoreError, request::CoreRequest, response::CoreResponse};
use crate::{cogs, runtimes::axum::extractors::auth::ExtractAuth};

pub async fn authorise(
    ExtractAuth(auth): ExtractAuth,
    request: Request,
) -> Result<CoreResponse, CoreError> {
    // Pass to internal handler
    let core_request = CoreRequest::from(request).with_auth(auth);
    cogs::authorise(core_request).await
}
