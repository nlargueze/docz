//! Generic AST for docz

mod conv;
mod error;

use std::collections::HashMap;

pub use conv::*;
pub use error::*;

/// AST node
#[derive(Debug, Clone)]
pub enum Node {
    Document {
        title: Option<String>,
        summary: Option<String>,
        authors: Option<Vec<String>>,
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    Fragment {
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    Chapter {
        title: Option<String>,
        span: Option<Span>,
        attrs: Attrs,
        children: Vec<Node>,
    },
    Section {
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    Heading {
        level: u8,
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    BlockQuote {
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    LineBreak {
        span: Option<Span>,
    },
    SoftBreak {
        span: Option<Span>,
    },
    CodeBlock {
        info: String,
        attrs: Attrs,
        span: Option<Span>,
        value: String,
    },
    Definition {
        id: String,
        label: String,
        url: String,
        title: Option<String>,
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    Italic {
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    Html {
        attrs: Attrs,
        span: Option<Span>,
        value: String,
    },
    Image {
        url: String,
        alt: String,
        title: Option<String>,
        attrs: Attrs,
        span: Option<Span>,
    },
    ImageRef {
        id: String,
        label: String,
        alt: String,
        attrs: Attrs,
        span: Option<Span>,
    },
    InlineCode {
        attrs: Attrs,
        span: Option<Span>,
        value: String,
    },
    Link {
        url: String,
        title: String,
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    LinkRef {
        id: String,
        label: String,
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    List {
        ordered: bool,
        start: Option<usize>,
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    ListItem {
        checked: Option<bool>,
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    Paragraph {
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    Bold {
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    Superscript {
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    Text {
        attrs: Attrs,
        span: Option<Span>,
        value: String,
    },
    ThematicBreak {
        attrs: Attrs,
        span: Option<Span>,
    },
    StrikeThrough {
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    FootnoteDef {
        id: String,
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    FootnoteRef {
        id: String,
        attrs: Attrs,
        span: Option<Span>,
    },
    Table {
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    TableRow {
        is_header: bool,
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    TableCell {
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    Metadata {
        attrs: Attrs,
        span: Option<Span>,
        value: String,
    },
    DescrList {
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    DescrItem {
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    DescrTerm {
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    DescrDetail {
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
    Comment {
        attrs: Attrs,
        span: Option<Span>,
        value: String,
    },
    Other {
        name: String,
        attrs: Attrs,
        span: Option<Span>,
        children: Vec<Node>,
    },
}

impl Node {
    /// Returns an immutable reference to the children
    pub fn children(&self) -> Option<&[Node]> {
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
    pub fn visit_and_modify(&self, f: &impl Fn(&Node) -> Option<Node>) -> Option<Self> {
        // NB: https://users.rust-lang.org/t/need-help-for-reached-the-recursion-limit-while-instantiating/86299
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
