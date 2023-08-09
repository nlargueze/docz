//! Traits

use crate::{Error, Node};

/// AST parser
pub trait AstParser {
    /// Extracts the AST
    fn parse(&self, data: &str) -> Result<Node, Error>;
}

/// AST transformer
pub trait AstTransformer {
    /// Renders an AST
    fn transform(&self, node: Node) -> Result<Node, Error>;
}

/// AST renderer
pub trait AstRenderer {
    /// Renders an AST
    fn render(&self, node: &Node) -> Result<String, Error>;
}
