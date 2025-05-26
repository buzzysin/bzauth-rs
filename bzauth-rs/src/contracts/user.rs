#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub image: Option<String>,
}
