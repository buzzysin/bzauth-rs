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
    pub fn to_u16(&self) -> u16 {
        match self {
            Status::Ok => 200,
            Status::Redirect => 302,
            Status::MovedPermanently => 301,
            Status::TemporaryRedirect => 307,
            Status::PermanentRedirect => 308,
            Status::BadRequest => 400,
            Status::Unauthorized => 401,
            Status::Forbidden => 403,
            Status::NotFound => 404,
            Status::InternalServerError => 500,
        }
    }
}
