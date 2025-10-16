//! Error types for the Manus AI backend

use thiserror::Error;

/// Main error type for the backend
#[derive(Error, Debug)]
pub enum ManusError {
    /// Sui blockchain interaction error
    #[error("Sui error: {0}")]
    Sui(String),

    /// Cryptography error
    #[error("Cryptography error: {0}")]
    Crypto(String),

    /// AI agent error
    #[error("Agent error: {0}")]
    Agent(String),

    /// Database error
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// API error
    #[error("API error: {0}")]
    Api(String),

    /// ZK proof error
    #[error("ZK proof error: {0}")]
    ZkProof(String),

    /// Generic error
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, ManusError>;

