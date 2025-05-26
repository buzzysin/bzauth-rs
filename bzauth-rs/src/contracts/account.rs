use super::token::Token;

#[derive(Debug, Clone, Default)]
pub struct Account {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub provider_id: Option<String>,
    pub token: Option<Token>,
}
