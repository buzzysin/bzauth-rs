use http::StatusCode;
use oauth2::{AuthorizationCode, StandardTokenResponse, TokenResponse};
use serde::{Deserialize, Serialize};

use crate::auth::{SignInOptions, SignInResult};
use crate::contracts::adapt::{AdaptAccount, AdaptUser, ProviderAccountId, SessionUser};
use crate::contracts::profile::Profile;
use crate::contracts::provide::{ProvideOAuth2, ProviderType};
use crate::contracts::token::Token;
use crate::tools::request::CoreRequest;
use crate::tools::response::CoreResponse;
use crate::tools::{CoreError, actions, generators};

impl<EF, TT> From<StandardTokenResponse<EF, TT>> for Token
where
    EF: oauth2::ExtraTokenFields,
    TT: oauth2::TokenType,
{
    fn from(token_response: StandardTokenResponse<EF, TT>) -> Self {
        // Serialize then deserialize to convert to the Token type
        serde_json::from_value(serde_json::to_value(token_response).unwrap()).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackRequest {
    code: String,
    state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackResponse {}

// Handle the callback
pub async fn callback(
    request: CoreRequest<CallbackRequest>,
) -> Result<CoreResponse<CallbackResponse>, CoreError> {
    let provider = request.extract_provider()?;
    let provider_type = provider.provider_type();

    // The user has just completed the OAuth2 flow and is being redirected back to the application
    // with an authorization code. We need to exchange this code for an access token.

    match provider_type {
        ProviderType::OAuth => callback_oauth2(request).await,
        ProviderType::Email => callback_email(request).await,
        ProviderType::Credentials => callback_credentials(request).await,
        ProviderType::OIDC => callback_oidc(request).await,
    }
}

async fn callback_oauth2(
    request: CoreRequest<CallbackRequest>,
) -> Result<CoreResponse<CallbackResponse>, CoreError> {
    /*
    Handle potential callback errors from the provider
    */
    if let Some(error) = request.query().get("error") {
        return Err(CoreError::new()
            .with_message(format!("OAuth2 error: {}", error))
            .with_status(StatusCode::BAD_REQUEST.into()));
    }

    /*
    Check if there is an existing session token in the request.
    */
    let adaptor = request.extract_adaptor()?;
    let session_token = request.extract_session_token().ok();
    let (session, _) = if let Some(ref session_token) = session_token {
        tracing::debug!("[callback] Session token found: {}", session_token);
        if let Some(SessionUser { session, user }) =
            adaptor.get_session_and_user(session_token.clone()).await
        {
            tracing::debug!(
                "[callback] Session and user found: {:?}",
                (session.clone(), user.clone())
            );
            (Some(session), Some(user))
        } else {
            tracing::error!("[callback] Failed to get session and user");
            (None, None)
        }
    } else {
        tracing::debug!("[callback] No session token found");
        (None, None)
    };

    /*
    Get the provider response and user profile.
    */
    let provider = request.extract_provider()?;
    let oauth2_provider = provider
        .as_ref()
        .as_oauth2()
        .ok_or_else(|| CoreError::new().with_message("Provider is not OAuth2"))?;

    let (profile_response, profile_user, adapt_account) =
        helpers::get_provider_response(&request, oauth2_provider).await?;

    /*
    Look for an account in the database that matches the provider account ID
    */
    let provider_id = oauth2_provider.id().clone();
    let profile_id = profile_response.id.clone();
    let profile_sub = profile_response.sub.clone();
    let provider_account_id = profile_id
        .or(profile_sub)
        .clone()
        .ok_or_else(|| CoreError::new().with_message("Provider account ID is missing"))?;
    let adapt_user = adaptor
        .get_user_by_account(ProviderAccountId {
            provider_id,
            provider_account_id,
        })
        .await;
    tracing::debug!("[callback] Adapted User: {:?}", adapt_user);

    /*
    Perform the user defined check to see if the user is allowed to sign in
    */
    let auth = request.extract_auth()?;
    if let Some(authorisation_redirect) = helpers::get_authorisation_redirect(
        &adapt_user.clone().or(Some(*profile_user.clone())),
        &adapt_account,
        &profile_response,
        auth.clone(),
    )
    .await
    {
        return authorisation_redirect;
    }

    // If the user is already authorised, redirect them to the home page
    let response = if let Some(adapt_user) = adapt_user {
        tracing::debug!("[callback] User already exists: {:?}", adapt_user);
        actions::sign_in(request.clone(), adapt_user, adapt_account, adaptor, session).await
    } else {
        tracing::debug!("[callback] Registering new user: {:?}", profile_user);
        actions::register(request.clone(), *profile_user, adapt_account, adaptor).await
    };

    let mut incoming_cookies = request.cookies().clone();
    if let Ok(_) = &response {
        incoming_cookies.set(
            "session",
            session_token.unwrap_or_else(|| {
                // Generate a new session token if it doesn't exist
                uuid::Uuid::new_v4().to_string()
            }),
        );
    }

    response.map(|r| {
        // Set the cookies in the response
        let mut cookies = r.cookies().clone();
        cookies.extend(incoming_cookies);
        r.with_cookies(cookies)
    })
}

// ignore
async fn callback_email(
    request: CoreRequest<CallbackRequest>,
) -> Result<CoreResponse<CallbackResponse>, CoreError> {
    // Handle email provider callback
    let provider = request.extract_provider()?;
    let provider_type = provider.provider_type();

    // So far unsupported
    Err(CoreError::new()
        .with_message(format!("Unsupported provider type: {}", provider_type))
        .with_status(StatusCode::BAD_REQUEST.into()))
}

// ignore
async fn callback_credentials(
    request: CoreRequest<CallbackRequest>,
) -> Result<CoreResponse<CallbackResponse>, CoreError> {
    // Handle credentials provider callback
    let provider = request.extract_provider()?;
    let provider_type = provider.provider_type();

    // So far unsupported
    Err(CoreError::new()
        .with_message(format!("Unsupported provider type: {}", provider_type))
        .with_status(StatusCode::BAD_REQUEST.into()))
}

// ignore
async fn callback_oidc(
    request: CoreRequest<CallbackRequest>,
) -> Result<CoreResponse<CallbackResponse>, CoreError> {
    // Handle OIDC provider callback
    let provider = request.extract_provider()?;
    let provider_type = provider.provider_type();

    // So far unsupported
    Err(CoreError::new()
        .with_message(format!("Unsupported provider type: {}", provider_type))
        .with_status(StatusCode::BAD_REQUEST.into()))
}

mod helpers {
    use std::sync::Arc;

    use super::*;
    use crate::auth::Auth;
    use crate::contracts::account::Account;
    use crate::contracts::user::User;

    /// Delegates to the user-defined sign in check callback if it exists.
    /// This will redirect the user if they are not allowed to sign in
    pub async fn get_authorisation_redirect(
        adapt_or_profile_user: &Option<AdaptUser>,
        adapt_account: &AdaptAccount,
        profile: &Profile,
        auth: Arc<Auth>,
    ) -> Option<Result<CoreResponse<CallbackResponse>, CoreError>> {
        let auth_sign_in = auth
            .options
            .callbacks
            .as_ref()
            .and_then(|c| c.sign_in.as_ref())?;

        tracing::debug!("[callback:sign_in] Running user defined sign in check");

        let sign_in_options = SignInOptions {
            user: adapt_or_profile_user.clone(),
            account: Some(adapt_account.clone()),
            profile: Some(profile.clone()),
        };

        match auth_sign_in(sign_in_options).await {
            SignInResult::Error(message) => {
                tracing::debug!(
                    "[callback:sign_in] User defined sign in check failed: {}",
                    message
                );
                Some(Err(CoreError::new()
                    .with_message(message)
                    .with_status(StatusCode::FORBIDDEN.into())))
            }
            SignInResult::Redirect(url) => {
                tracing::debug!(
                    "[callback:sign_in] User defined sign in check redirecting to: {}",
                    url
                );
                Some(Ok(CoreResponse::redirect(url)))
            }
            SignInResult::Success => {
                tracing::debug!("[callback:sign_in] User defined sign in check succeeded");
                None
            }
        }
    }

    pub async fn get_provider_response(
        request: &CoreRequest<CallbackRequest>,
        oauth2_provider: &dyn ProvideOAuth2,
    ) -> Result<(Profile, Box<User>, Account), CoreError> {
        let code = AuthorizationCode::new(request.extract_code()?);
        let client = generators::generate_client_from_provider(oauth2_provider)?;

        /*
        Exchange the authorization code for an access token.
         */
        let token_response = client
            .exchange_code(code)
            .request_async(&generators::generate_http_client()?)
            .await
            .map_err(|e| {
                CoreError::new()
                    .with_message(format!("Failed to exchange code: (error={})", e))
                    .with_status(StatusCode::BAD_REQUEST.into())
            })?;
        tracing::debug!("[callback] Token: {:?}", token_response.access_token());

        /*
        Get the user profile from the provider using the access token.
         */
        let profile_endpoint = oauth2_provider.profile_endpoint();
        let profile_client = generators::generate_http_client()?;
        let profile_response = profile_client
        .get(profile_endpoint.url())
        .bearer_auth(token_response.access_token().secret())
        .send()
        .await
        .map_err(|e| {
            CoreError::new()
                .with_message(format!("Failed to fetch user info: (error={})", e))
                .with_status(StatusCode::BAD_REQUEST.into())
        })?
        .json::<Profile>()// TODO: Change to a proper user info type
        .await
        .map_err(|e| {
            CoreError::new()
                .with_message(format!("Failed to parse user info: (error={})", e))
                .with_status(StatusCode::BAD_REQUEST.into())
        })?;
        tracing::debug!("[callback] User Info: {:?}", profile_response);

        let profile_user = {
            let mut profile_user = oauth2_provider.get_profile(profile_response.clone());
            profile_user.id = Some(uuid::Uuid::new_v4().to_string());
            profile_user.email = profile_response.email.clone();
            profile_user
        };

        /*
           Prepare the AdaptAccount to be stored in the database.
        */
        let adapt_token = Token::from(token_response.clone());
        let adapt_account_id = uuid::Uuid::new_v4().to_string();
        let adapt_provider_id = oauth2_provider.id().to_string();
        let adapt_provider_type = oauth2_provider.provider_type();
        let adapt_account = AdaptAccount {
            id: Some(adapt_account_id.clone()),
            user_id: profile_user.id.clone(),
            provider_id: Some(adapt_provider_id.clone()),
            provider_type: adapt_provider_type,
            // For OAuth2, the provider account ID is usually the user ID/Sub from the provider
            provider_account_id: Some(
                vec![profile_response.id.clone(), profile_response.sub.clone()]
                    .into_iter()
                    .filter_map(|id| id)
                    .next()
                    .ok_or_else(|| {
                        CoreError::new()
                            .with_message("Provider account ID is missing")
                            .with_status(StatusCode::BAD_REQUEST.into())
                    })?,
            ),
            token: Some(adapt_token),
        };
        tracing::debug!("[callback] Adapted Account: {:?}", adapt_account);

        Ok((profile_response, profile_user, adapt_account))
    }
}
