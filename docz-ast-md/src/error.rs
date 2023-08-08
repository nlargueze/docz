//! Error

/// Error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid data
    #[error("invalid format: {0}")]
    Invalid(String),
}
