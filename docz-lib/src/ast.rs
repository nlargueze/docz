//! AST

use std::collections::HashMap;

/// Node type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum NodeType {
    /// Root
    #[default]
    Root,
    /// Text node
    Text(String),
    /// Comment node
    Comment(String),
}

/// AST node
#[derive(Debug, Default)]
pub struct Node {
    /// Node type
    pub ty: NodeType,
    // Tag name
    pub tag: Option<String>,
    /// Attributes
    pub attributes: HashMap<String, String>,
    /// Children
    pub children: Vec<Node>,
}

impl Node {
    /// Creates a root node
    pub fn root() -> Self {
        Self::default()
    }

    /// Creates a text node
    pub fn text(text: &str) -> Self {
        Self {
            ty: NodeType::Text(text.to_string()),
            children: vec![],
        }
    }

    /// Creates a comment node
    pub fn comment(comment: &str) -> Self {
        Self {
            ty: NodeType::Comment(comment.to_string()),
            children: vec![],
        }
    }
}
