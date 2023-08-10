//! Error

use crate::{Node, Position};

/// AST error
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct Error {
    /// Error message
    pub message: String,
    /// Optional position
    pub position: Option<Position>,
    /// Optional node
    pub node: Option<Node>,
}

impl Error {
    /// Creates a new error
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
            position: None,
            node: None,
        }
    }

    /// Assigns a position to the error
    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }

    /// Assigns a node to the error
    pub fn node(mut self, node: Node) -> Self {
        self.node = Some(node);
        self
    }
}
