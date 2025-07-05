use crate::contracts::account::Account;
use crate::contracts::adapt::Adapt;
use crate::contracts::provide::Provide;
use crate::contracts::user::User;
use crate::tools::request::CoreRequest;
use crate::tools::response::CoreResponse;
use crate::tools::{CallbackRequest, CallbackResponse, CoreError};

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
