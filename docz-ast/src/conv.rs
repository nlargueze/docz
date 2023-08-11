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
    fn process(&self, nodes: Vec<Node>) -> Result<Vec<Node>, Error>;
}

/// AST renderer
pub trait Renderer {
    /// Renders an AST to a string
    fn render(&self, node: &Node) -> Result<String, Error>;
}

/// Debug renderer
///
/// This renderer prints the AST in a debug format
#[derive(Debug, Default)]
pub struct DebugRenderer {}

impl Renderer for DebugRenderer {
    fn render(&self, node: &Node) -> Result<String, Error> {
        Ok(format!("{node:#?}"))
    }
}
