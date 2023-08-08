//! Generic AST for docz

use std::{collections::HashMap, error::Error};

/// AST node
#[derive(Debug, Default, Clone)]
pub struct Node {
    /// Type
    pub ty: NodeType,
    /// Tag
    pub tag: Option<String>,
    /// Attributes
    pub attributes: HashMap<String, String>,
    /// Content
    pub content: Option<String>,
    /// Children
    pub children: Vec<Node>,
}

/// Node kind
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum NodeType {
    #[default]
    Root,
    FrontMatter,
    Text,
    Comment,
    Code,
    CodeBlock {
        lang: String,
    },
    LineBreak,
    SoftBreak,
    ThematicBreak,
    Heading(u8),
    Italic,
    Bold,
    StrikeThrough,
    Paragraph,
    UnorderedList,
    OrderedList {
        start: usize,
    },
    ListItem,
    Image {
        url: String,
        title: String,
    },
    Link {
        url: String,
        title: String,
    },
    FootnoteRef(String),
    Footnote,
}

impl Node {
    /// Sets the node type
    pub fn ty(&mut self, ty: NodeType) {
        self.ty = ty;
    }

    /// Sets the node tag
    pub fn tag(&mut self, tag: &str) {
        self.tag = Some(tag.to_string());
    }

    /// Adds a node atttribute
    pub fn attr(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    /// Sets the node content
    pub fn content(&mut self, data: &str) {
        self.content = Some(data.to_string());
    }

    /// Adds a node child
    pub fn child(&mut self, child: Node) {
        self.children.push(child);
    }

    /// Creates a root node
    pub fn root() -> Self {
        Self::default()
    }
}

/// AST parser
pub trait Parser {
    /// Error
    type Err: Error;

    /// Extracts the AST
    fn parse(&self, data: &str) -> Result<Node, Self::Err>;
}

/// AST renderer
pub trait Renderer {
    /// Error
    type Err: Error;

    /// Renders an AST
    fn render(&self, node: &Node) -> Result<String, Self::Err>;
}
