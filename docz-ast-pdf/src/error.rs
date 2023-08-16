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

impl From<printpdf::Error> for Error {
    fn from(err: printpdf::Error) -> Self {
        Self {
            message: format!("{}", err),
        }
    }
}
