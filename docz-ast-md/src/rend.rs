//! Renderer

use docz_ast::{AstRenderer, Error, Node};

/// Rendering error
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    /// Invalid input
    #[error("invalid: {0}")]
    Invalid(String),
}

/// AST renderer for markdown
#[derive(Debug, Default)]
pub struct MdRenderer {}

impl MdRenderer {
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }
}

impl AstRenderer for MdRenderer {
    fn render(&self, _node: &Node) -> Result<String, Error> {
        // let mut data = String::new();
        todo!("render to Markdown");
    }
}
