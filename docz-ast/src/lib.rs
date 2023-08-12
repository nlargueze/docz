//! Generic AST for docz

mod conv;
mod error;
mod json;

use std::collections::HashMap;

pub use conv::*;
pub use error::*;
use serde::Serialize;

/// AST node
#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub attrs: HashMap<String, Option<String>>,
    pub span: Option<Span>,
    pub value: Option<String>,
    pub children: Option<Vec<Node>>,
}

impl Node {
    /// Creates a new node
    pub fn new(kind: NodeKind) -> Self {
        Self {
            kind,
            attrs: HashMap::new(),
            span: None,
            children: None,
            value: None,
        }
    }

    /// Sets the node kind
    pub fn set_kind(&mut self, kind: NodeKind) -> &mut Self {
        self.kind = kind;
        self
    }

    /// Sets the node span
    pub fn set_span(&mut self, span: Span) -> &mut Self {
        self.span = Some(span);
        self
    }

    /// Adds the node attribute
    pub fn add_attr(&mut self, key: &str, value: Option<&str>) -> &mut Self {
        self.attrs
            .insert(key.to_string(), value.map(|v| v.to_string()));
        self
    }

    /// Sets the node value
    pub fn set_value(&mut self, value: &str) -> &mut Self {
        self.value = Some(value.to_string());
        self
    }

    /// Adds a node child
    pub fn add_child(&mut self, node: Node) -> &mut Self {
        if let Some(children) = &mut self.children {
            children.push(node);
        } else {
            self.children = Some(vec![node]);
        }
        self
    }
}

/// AST node kind
#[derive(Debug, Clone)]
pub enum NodeKind {
    Document,
    Fragment,
    FrontMatter,
    Chapter,
    Section,
    Heading { level: u8, id: Option<String> },
    Paragraph,
    Text,
    Comment,
    ThematicBreak,
    LineBreak,
    SoftBreak,
    Italic,
    Bold,
    BlockQuote,
    List { ordered: bool },
    ListItem { index: Option<usize> },
    Code,
    Link { url: String, title: Option<String> },
    Image { url: String, title: Option<String> },
    Html,
    Table,
    TableRow { is_header: bool },
    TableCell,
    CodeBlock { info: String },
    FootnoteRef { id: String },
    FootnoteDef { id: String },
    DefinitionList,
    DefinitionItem,
    DefinitionTerm,
    DefinitionDetails,
    StrikeThrough,
    TaskItem { checked: bool },
    Highlight,
    SubScript,
    SuperScript,
    Other { name: String },
}

/// AST Span
#[derive(Debug, Clone, Serialize)]
pub struct Span {
    /// Start line
    pub start_line: usize,
    /// Start column
    pub start_col: usize,
    /// End line
    pub end_line: usize,
    /// End column
    pub end_col: usize,
}

impl Span {
    /// Creates a new span
    pub fn new(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }
}
