//! Conversion traits

use crate::{Error, Node, NodeData};

/// AST converter
pub trait Converter<T, U>
where
    T: NodeData,
    U: NodeData,
{
    /// Converts from an AST to another
    fn convert(&self, node: Node<T>) -> Result<Node<U>, Error>;
}

/// AST parser
pub trait Parser<T>
where
    T: NodeData,
{
    /// Parses to an AST
    fn parse(&self, value: &[u8]) -> Result<Node<T>, Error>;
}

/// AST renderer
pub trait Renderer<T>
where
    T: NodeData,
{
    /// Renders an AST to bytes
    fn render(&self, node: &Node<T>) -> Result<Vec<u8>, Error>;
}
