//! Conversion traits

use crate::{Error, Node};

/// AST parser
pub trait Parser {
    /// Parses a string to its AST
    fn parse(&self, value: &str) -> Result<Node, Error>;
}

/// AST processor
pub trait Processor {
    /// Processes a node
    fn process(&self, node: Node) -> Result<Node, Error>;
}

/// AST renderer
pub trait Renderer {
    /// Renders an AST to a string
    fn render(&self, node: &Node) -> Result<String, Error>;
}
