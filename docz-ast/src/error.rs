//! Error

use crate::Node;

/// AST error
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct Error {
    /// Error message
    pub message: String,
    /// Error source
    pub src: Option<String>,
    /// Error code
    pub node: Option<Node>,
}

impl Error {
    /// Creates a new error
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
            src: None,
            node: None,
        }
    }

    /// Assigns a source to the error
    pub fn source(mut self, source: &str) -> Self {
        self.src = Some(source.to_string());
        self
    }

    /// Assigns an AST node to the error
    pub fn node(mut self, node: Node) -> Self {
        self.node = Some(node);
        self
    }
}
