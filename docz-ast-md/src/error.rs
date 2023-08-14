//! Error

use std::string::FromUtf8Error;

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

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::new(&format!("{}", err))
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Self::new(&format!("{}", err))
    }
}
