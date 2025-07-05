use std::sync::Arc;

use crate::auth::Auth;
use crate::contracts::account::Account;
use crate::contracts::adapt::Adapt;
use crate::contracts::provide::Provide;
use crate::contracts::user::User;
use crate::tools::request::CoreRequest;
use crate::tools::response::CoreResponse;
use crate::tools::{CallbackRequest, CallbackResponse, CoreError};

pub async fn sign_in(
    request: CoreRequest<CallbackRequest>,
    _adapt_user: Option<User>,
    _adapt_account: Option<Account>,
    _provider: &dyn Provide,
    _adaptor: &dyn Adapt,
    auth: Arc<Auth>,
) -> Result<CoreResponse<CallbackResponse>, CoreError> {
    // Infer the host from the request headers
    let url = request.uri().to_string();
    let host = request.headers().get("host").and_then(|h| h.to_str().ok());
    if host.is_none() {
        return Err(
            CoreError::new().with_message("Failed to infer the host from the request headers")
        );
    }

    // Convert the host to a string
    let host = host.unwrap().to_string();

    let redirect_callback = auth.options.callbacks.as_ref().map(|c| c.redirect.as_ref());

    if redirect_callback.is_none() {
        return Err(CoreError::new().with_message("No redirect callback defined"));
    }
    let redirect_callback = redirect_callback.unwrap();

    let redirect_url = redirect_callback(url.clone(), host).await;

    Ok(CoreResponse::new().with_redirect(redirect_url))
}
