use crate::contracts::account::Account;
use crate::contracts::adapt::{Adapt, CreateSessionOptions};
use crate::contracts::user::User;
use crate::tools::request::CoreRequest;
use crate::tools::response::CoreResponse;
use crate::tools::{CallbackRequest, CallbackResponse, CoreError};

/// # Panics
/// This function will panic if the `profile_user.id` is `None` after user creation
/// or if there are issues generating the session token. THIS IS A BUG AND SHOULD BE FIXED.
pub async fn register(
    request: CoreRequest<CallbackRequest>,
    profile_user: User,
    adapt_account: Account,
    adaptor: &dyn Adapt,
) -> Result<CoreResponse<CallbackResponse>, CoreError> {
    let user_email = profile_user.email.clone();

    // Check if email is already registered
    let user_by_email = if let Some(email) = user_email {
        adaptor.get_user_by_email(email.clone()).await
    } else {
        None
    };

    // If the user already exists by email, return an error
    if user_by_email.is_some() {
        return Err(CoreError::new().with_message("Email is already registered"));
    }

    // Create user, link account, generate session, and redirect
    let session_generated = uuid::Uuid::new_v4(); // Generate a new session token

    let user = adaptor.create_user(profile_user.clone()).await;
    tracing::debug!("[callback:register] Created User: {:?}", user);

    let account = adaptor.link_account(adapt_account.clone()).await;
    tracing::debug!("[callback:register] Linked Account: {:?}", account);

    let session = adaptor
        .create_session(CreateSessionOptions {
            token: session_generated.to_string(), // TODO: Generate a proper token
            user_id: profile_user.id.clone().unwrap(),
            expires_in: 3600, // TODO: Set appropriate expiration time from configuration
        })
        .await;
    tracing::debug!("[callback:register] Created Session: {session:?}");

    let mut cookies = request.cookies().clone();
    cookies.set("session", session_generated.to_string());

    #[cfg(debug_assertions)]
    println!("[register] Cookies after registration: {cookies:?}");

    // Infer the host from the request headers
    let redirect_url = request.extract_redirect_url().await?;

    // TODO: If a callback-url cookie is set, use that instead of redirecting to the home page
    Ok(CoreResponse::new()
        .with_redirect(&redirect_url)
        .with_cookies(cookies.clone()))
}
