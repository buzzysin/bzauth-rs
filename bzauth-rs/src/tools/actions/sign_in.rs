use crate::contracts::account::Account;
use crate::contracts::adapt::{Adapt, AdaptSession, CreateSessionOptions};
use crate::contracts::user::User;
use crate::tools::request::CoreRequest;
use crate::tools::response::CoreResponse;
use crate::tools::{CallbackRequest, CallbackResponse, CoreError};

pub async fn sign_in(
    request: CoreRequest<CallbackRequest>,
    adapt_user: User,
    _adapt_account: Account,
    adaptor: &dyn Adapt,
    session: Option<AdaptSession>, // auth: Arc<Auth>,
) -> Result<CoreResponse<CallbackResponse>, CoreError> {
    // Create or define a new session for the user

    // If there is no session, generate a new one
    let _session = if let Some(session) = session {
        tracing::debug!("[callback] Using existing session: {:?}", session);
        session
    } else {
        let session_token = uuid::Uuid::new_v4().to_string(); // Replace with actual session token generation logic

        adaptor
            .create_session(CreateSessionOptions {
                user_id: adapt_user.id.clone().unwrap(),
                token: session_token.to_string(),
                expires_in: 60 * 60, // Set expiration time to 1 hour (3600 seconds)
            })
            .await
            .ok_or_else(|| CoreError::new().with_message("Failed to create session"))?
    };

    let redirect_url = request.extract_redirect_url().await?;

    Ok(CoreResponse::new()
        .with_redirect(redirect_url)
        .with_cookie("session".to_string(), _session.token.clone()))
}
