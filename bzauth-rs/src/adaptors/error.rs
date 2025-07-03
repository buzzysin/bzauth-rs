#[derive(Debug, Clone)]
pub struct AdaptorError {
    pub message: String,
}

impl AdaptorError {
    pub fn new(message: String) -> Self {
        AdaptorError { message }
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.message = message;
        self
    }
}

impl std::fmt::Display for AdaptorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AdaptorError: {}", self.message)
    }
}

impl std::error::Error for AdaptorError {}
