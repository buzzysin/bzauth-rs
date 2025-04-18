#[derive(Debug, Clone)]
pub enum ProviderError {
    MissingClientId(String),
    MissingClientSecret(String),
}
