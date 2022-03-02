use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid private key length")]
    InvalidPrivateKeyLength,
    #[error("Invalid public key length")]
    InvalidPublicKeyLength,
    #[error("Invalid public key format (expected hex)")]
    InvalidPublicKeyFormat,
}
