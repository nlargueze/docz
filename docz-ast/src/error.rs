//! Error

use crate::{Node, Span};

/// AST error
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct Error {
    /// Error message
    pub message: String,
    /// Optional span
    pub span: Option<Span>,
    /// Optional node
    pub node: Option<Node>,
}

impl Error {
    /// Creates a new error
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
            span: None,
            node: None,
        }
    }

    /// Assigns a span to the error
    pub fn position(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    /// Assigns a node to the error
    pub fn node(mut self, node: Node) -> Self {
        self.node = Some(node);
        self
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
