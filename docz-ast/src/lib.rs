//! Generic AST for docz

use std::collections::HashMap;

mod error;
mod traits;

pub use error::*;
pub use traits::*;

/// A node
#[derive(Debug, Clone, Default)]
pub struct Node {
    /// Node type
    pub ty: NodeType,
    /// Node attributes
    pub attrs: HashMap<String, Option<String>>,
    /// Node value
    pub value: Option<String>,
    /// Node children
    pub children: Vec<Node>,
}

/// AST node
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum NodeType {
    /// Generic
    #[default]
    Generic,
    /// Document
    Document {
        title: Option<String>,
        authors: Vec<String>,
    },
    /// Document fragment
    Fragment,
    /// Chapter
    Chapter {
        title: Option<String>,
    },
    /// Page
    Page,
    Section,
    Heading {
        level: u8,
    },
    Paragraph,
    Row,
    PageBreak,
    LineBreak,
    SoftBreak,
    Divider,
    List {
        /// if true, the list is ordered
        ordered: bool,
        /// For ordered list, the start index
        start: Option<usize>,
    },
    ListItem,
    Table,
    TableRow,
    FootnoteRef,
    Footnote,
    DescrList,
    DescrItem,
    DescrTerm,
    DescrDetails,
    Link {
        url: String,
        title: Option<String>,
    },
    Image {
        url: String,
        title: Option<String>,
    },
    CodeBlock {
        lang: Option<String>,
    },
    BlockQuote,
    HtmlBlock,
    Text,
    Comment,
    Italic,
    Bold,
    StrikeThrough,
    Code,
}

impl Node {
    /// Adds a child to the node
    pub fn add_child(&mut self, child: Node) -> &mut Self {
        self.children.push(child);
        self
    }

    /// Adds an attribute to the node
    pub fn add_attr(&mut self, key: &str, value: Option<&str>) -> &mut Self {
        self.attrs
            .insert(key.to_string(), value.map(|v| v.to_string()));
        self
    }

    /// Sets the node type
    pub fn set_type(&mut self, ty: NodeType) -> &mut Self {
        self.ty = ty;
        self
    }

    /// Sets the node value
    pub fn set_value(&mut self, value: &str) -> &mut Self {
        self.value = Some(value.to_string());
        self
    }
}
