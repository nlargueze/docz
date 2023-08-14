//! Error

/// AST error
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct Error {
    /// Error message
    pub message: String,
}

impl Error {
    /// Creates a new error
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::new(value.to_string().as_str())
    }
}

impl From<std::fmt::Error> for Error {
    fn from(value: std::fmt::Error) -> Self {
        Error::new(value.to_string().as_str())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::new(value.to_string().as_str())
    }
}
