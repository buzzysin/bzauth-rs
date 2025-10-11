#[derive(Debug, Clone)]
pub enum Status {
    // Success
    Ok = 200,
    // Redirect
    Redirect = 302,
    MovedPermanently = 301,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
    // Client errors
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    // Server errors
    InternalServerError = 500,
}

impl Status {
    pub const fn to_u16(&self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::Redirect => 302,
            Self::MovedPermanently => 301,
            Self::TemporaryRedirect => 307,
            Self::PermanentRedirect => 308,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::InternalServerError => 500,
        }
    }
}
