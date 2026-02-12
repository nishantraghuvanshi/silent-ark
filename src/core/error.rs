//! Error types for Silent Ark

use thiserror::Error;

/// Main error type for Silent Ark operations
#[derive(Error, Debug)]
pub enum SilentArkError {
    /// Silent Payments encoding/decoding error
    #[error("Silent Payments encoding error: {0}")]
    SilentPaymentsEncoding(String),

    /// Cryptographic operation failed
    #[error("Cryptographic error: {0}")]
    Crypto(String),

    /// vTXO not found or invalid
    #[error("vTXO error: {0}")]
    VTXO(String),

    /// Ark protocol error
    #[error("Ark protocol error: {0}")]
    Ark(String),

    /// Feature not yet implemented
    #[error("Not implemented")]
    NotImplemented,

    /// Invalid input parameters
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Blockchain scanning error
    #[error("Scanning error: {0}")]
    Scanning(String),

    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// IO error
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Result type alias for Silent Ark operations
pub type Result<T> = std::result::Result<T, SilentArkError>;
