use crate::{
    contracts::{account::Account, adapt::Adapt, provide::Provide, user::User},
    tools::{
        CallbackRequest, CallbackResponse, CoreError, request::CoreRequest, response::CoreResponse,
    },
};

pub async fn sign_in(
    _request: CoreRequest<CallbackRequest>,
    _adapt_user: Option<User>,
    _adapt_account: Option<Account>,
    _provider: &dyn Provide,
    _adaptor: &dyn Adapt,
    // auth: Arc<Auth>,
) -> Result<CoreResponse<CallbackResponse>, CoreError> {
    Ok(CoreResponse::new().with_redirect("http://localhost:3000/".to_string()))
}
