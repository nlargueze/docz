//! Conversion traits

use crate::{Error, Node};

/// AST parser
pub trait Parser {
    /// Parses a string to an AST
    fn parse(&self, value: &str) -> Result<Node, Error>;
}

/// AST processor
pub trait Processor {
    /// Processes AST nodes
    fn process(&self, nodes: Vec<Node>) -> Result<Vec<Node>, Error>;
}

/// AST renderer
pub trait Renderer {
    /// Checks if the output is binary
    fn is_binary(&self) -> bool;

    /// Renders an AST to bytes
    fn render(&self, node: &Node) -> Result<Vec<u8>, Error>;

    /// Renders an AST to string
    fn render_str(&self, node: &Node) -> Result<String, Error> {
        let bytes = self.render(node)?;
        let node_str =
            String::from_utf8(bytes).map_err(|err| Error::new(err.to_string().as_str()))?;
        Ok(node_str)
    }
}

/// Debug renderer
///
/// This renderer prints the AST in a debug format
#[derive(Debug, Default)]
pub struct DebugRenderer {}

impl Renderer for DebugRenderer {
    fn is_binary(&self) -> bool {
        false
    }

    fn render(&self, node: &Node) -> Result<Vec<u8>, Error> {
        Ok(format!("{node:#?}").into_bytes())
    }
}
