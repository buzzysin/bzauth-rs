use super::user::User;

#[derive(Debug, Clone, Default)]
pub struct Session {
    pub user: Option<User>,
    pub expires_at: Option<u64>,
}
