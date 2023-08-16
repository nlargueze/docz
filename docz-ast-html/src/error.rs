//! Error

/// Parsing error
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct Error {
    /// Message
    pub message: String,
}

impl Error {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

impl From<html_parser::Error> for Error {
    fn from(value: html_parser::Error) -> Self {
        Self::new(&value.to_string())
    }
}
