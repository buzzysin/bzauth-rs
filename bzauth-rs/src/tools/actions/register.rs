use crate::contracts::account::Account;
use crate::contracts::adapt::{Adapt, CreateSessionOptions};
use crate::contracts::provide::Provide;
use crate::contracts::user::User;
use crate::tools::request::CoreRequest;
use crate::tools::response::CoreResponse;
use crate::tools::{CallbackRequest, CallbackResponse, CoreError};

pub async fn register(
    _request: CoreRequest<CallbackRequest>,
    _user: Option<User>,
    _account: Option<Account>,
    _provider: &dyn Provide,
    _adaptor: &dyn Adapt,
    // auth: Arc<Auth>,
) -> Result<CoreResponse<CallbackResponse>, CoreError> {
    if _user.is_none() {
        return Err(CoreError::new().with_message("User is required for registration".to_string()));
    }

    let _user = _user.unwrap();
    let user_email = _user.email.clone();

    // Check if email is already registered
    let user_by_email = if let Some(email) = user_email {
        _adaptor.get_user_by_email(email.clone()).await
    } else {
        None
    };

    // If the user already exists by email, return an error
    if user_by_email.is_some() {
        return Err(CoreError::new().with_message("Email is already registered".to_string()));
    }

    // Create user, link account, generate session, and redirect
    let session_generated = "TODO";

    let _debug = _adaptor.create_user(_user.clone()).await;
    tracing::debug!("[register] Created User: {:?}", _debug);

    let _debug = _adaptor.link_account(_account.unwrap()).await;
    tracing::debug!("[register] Linked Account: {:?}", _debug);

    let _debug = _adaptor
        .create_session(CreateSessionOptions {
            token: session_generated.to_string(), // TODO: Generate a proper token
            user_id: _user.id.clone().unwrap(),
            expires_in: 3600, // TODO: Set appropriate expiration time from configuration
        })
        .await;
    tracing::debug!("[register] Created Session: {:?}", _debug);

    let mut cookies = _request.cookies().clone();
    cookies.set("session_token", session_generated.to_string());

    // TODO: If a callback-url cookie is set, use that instead of redirecting to the home page
    Ok(CoreResponse::new()
        .with_redirect("http://localhost:3000/".to_string())
        .with_cookies(cookies.clone()))
}
