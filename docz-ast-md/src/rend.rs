//! Renderer

use docz_ast::{Error, Node, Renderer};

/// AST renderer for markdown
#[derive(Debug, Default)]
pub struct MdRenderer {}

impl MdRenderer {
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }
}

impl Renderer for MdRenderer {
    fn render(&self, _node: &Node) -> Result<Vec<u8>, Error> {
        todo!("render to Markdown");
        // let mut data = String::new();
    }
}
