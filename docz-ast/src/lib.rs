//! Generic AST for docz

mod conv;
mod error;

use std::collections::HashMap;

pub use conv::*;
pub use error::*;

#[derive(Debug, Clone)]
pub enum Node {
    Document {
        span: Option<Span>,
        children: Vec<Node>,
        attrs: Attrs,
        title: Option<String>,
        summary: Option<String>,
        authors: Option<Vec<String>>,
    },
    Fragment {
        span: Option<Span>,
        children: Vec<Node>,
        attrs: Attrs,
    },
    Chapter {
        span: Option<Span>,
        children: Vec<Node>,
        attrs: Attrs,
    },
    Section {
        span: Option<Span>,
        children: Vec<Node>,
        attrs: Attrs,
    },
    Heading {
        span: Option<Span>,
        children: Vec<Node>,
        attrs: Attrs,
        level: u8,
    },
    BlockQuote {
        span: Option<Span>,
        children: Vec<Node>,
        attrs: Attrs,
    },
    LineBreak {
        span: Option<Span>,
    },
    SoftBreak {
        span: Option<Span>,
    },
    CodeBlock {
        span: Option<Span>,
        value: String,
        attrs: Attrs,
        info: String,
    },
    Definition {
        span: Option<Span>,
        children: Vec<Node>,
        attrs: Attrs,
        id: String,
        label: String,
        url: String,
        title: Option<String>,
    },
    Italic {
        span: Option<Span>,
        children: Vec<Node>,
        attrs: Attrs,
    },
    Html {
        span: Option<Span>,
        value: String,
        attrs: Attrs,
    },
    Image {
        span: Option<Span>,
        attrs: Attrs,
        url: String,
        alt: String,
        title: Option<String>,
    },
    ImageRef {
        span: Option<Span>,
        attrs: Attrs,
        id: String,
        label: String,
        alt: String,
    },
    InlineCode {
        span: Option<Span>,
        attrs: Attrs,
        value: String,
    },
    Link {
        span: Option<Span>,
        attrs: Attrs,
        children: Vec<Node>,
        url: String,
        title: String,
    },
    LinkRef {
        span: Option<Span>,
        attrs: Attrs,
        children: Vec<Node>,
        id: String,
        label: String,
    },
    List {
        span: Option<Span>,
        attrs: Attrs,
        children: Vec<Node>,
        ordered: bool,
        start: Option<usize>,
    },
    ListItem {
        span: Option<Span>,
        attrs: Attrs,
        children: Vec<Node>,
        checked: Option<bool>,
    },
    Paragraph {
        span: Option<Span>,
        attrs: Attrs,
        children: Vec<Node>,
    },
    Bold {
        span: Option<Span>,
        attrs: Attrs,
        children: Vec<Node>,
    },
    Superscript {
        span: Option<Span>,
        attrs: Attrs,
        children: Vec<Node>,
    },
    Text {
        span: Option<Span>,
        attrs: Attrs,
        value: String,
    },
    ThematicBreak {
        span: Option<Span>,
        attrs: Attrs,
    },
    StrikeThrough {
        span: Option<Span>,
        children: Vec<Node>,
        attrs: Attrs,
    },
    FootnoteDef {
        span: Option<Span>,
        attrs: Attrs,
        children: Vec<Node>,
        id: String,
    },
    FootnoteRef {
        span: Option<Span>,
        attrs: Attrs,
        id: String,
    },
    Table {
        span: Option<Span>,
        attrs: Attrs,
        children: Vec<Node>,
    },
    TableRow {
        span: Option<Span>,
        attrs: Attrs,
        children: Vec<Node>,
        is_header: bool,
    },
    TableCell {
        span: Option<Span>,
        attrs: Attrs,
        children: Vec<Node>,
    },
    Metadata {
        span: Option<Span>,
        attrs: Attrs,
        value: String,
    },
    DescrList {
        span: Option<Span>,
        attrs: Attrs,
        children: Vec<Node>,
    },
    DescrItem {
        span: Option<Span>,
        attrs: Attrs,
        children: Vec<Node>,
    },
    DescrTerm {
        span: Option<Span>,
        attrs: Attrs,
        children: Vec<Node>,
    },
    DescrDetail {
        span: Option<Span>,
        attrs: Attrs,
        children: Vec<Node>,
    },
    Comment {
        span: Option<Span>,
        attrs: Attrs,
        value: String,
    },
    Other {
        span: Option<Span>,
        attrs: Attrs,
        children: Vec<Node>,
        name: String,
    },
}

impl Node {
    /// Returns a mutable reference to the children
    pub fn children_mut(&mut self) -> Option<&mut Vec<Node>> {
        match self {
            Node::Document { children, .. } => Some(children),
            Node::Fragment { children, .. } => Some(children),
            Node::Chapter { children, .. } => Some(children),
            Node::Section { children, .. } => Some(children),
            Node::Heading { children, .. } => Some(children),
            Node::BlockQuote { children, .. } => Some(children),
            Node::LineBreak { .. } => None,
            Node::CodeBlock { .. } => None,
            Node::Definition { children, .. } => Some(children),
            Node::Italic { children, .. } => Some(children),
            Node::Html { .. } => None,
            Node::Image { .. } => None,
            Node::ImageRef { .. } => None,
            Node::InlineCode { .. } => None,
            Node::Link { children, .. } => Some(children),
            Node::LinkRef { children, .. } => Some(children),
            Node::List { children, .. } => Some(children),
            Node::ListItem { children, .. } => Some(children),
            Node::Paragraph { children, .. } => Some(children),
            Node::Bold { children, .. } => Some(children),
            Node::Text { .. } => None,
            Node::ThematicBreak { .. } => None,
            Node::StrikeThrough { children, .. } => Some(children),
            Node::FootnoteDef { children, .. } => Some(children),
            Node::FootnoteRef { .. } => None,
            Node::Table { children, .. } => Some(children),
            Node::TableRow { children, .. } => Some(children),
            Node::TableCell { children, .. } => Some(children),
            Node::Metadata { .. } => None,
            Node::Other { children, .. } => Some(children),
            Node::SoftBreak { .. } => None,
            Node::Superscript { children, .. } => Some(children),
            Node::DescrList { children, .. } => Some(children),
            Node::DescrItem { children, .. } => Some(children),
            Node::DescrTerm { children, .. } => Some(children),
            Node::DescrDetail { children, .. } => Some(children),
            Node::Comment { .. } => None,
        }
    }

    /// Traverses and converts a node recursively
    ///
    /// If `None` is returned, the node is removed from the tree.
    pub fn visit_and_modify(&self, f: fn(&Node) -> Option<Node>) -> Option<Self> {
        let mut new_node = f(self);

        if let Some(new_node) = new_node.as_mut() {
            if let Some(children) = new_node.children_mut() {
                let mut new_children = vec![];
                for child in children.iter() {
                    if let Some(new_child) = child.visit_and_modify(f) {
                        new_children.push(new_child);
                    }
                }
                children.clear();
                children.append(&mut new_children);
            }
        }

        new_node
    }
}

/// Node attributes
pub type Attrs = HashMap<String, Option<String>>;

/// AST Span
#[derive(Debug, Clone)]
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
