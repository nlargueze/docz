//! Generic AST for docz

mod conv;
mod error;

use std::collections::HashMap;

pub use conv::*;
pub use error::*;

#[derive(Debug, Clone)]
pub enum Node {
    Document {
        position: Option<Position>,
        children: Vec<Node>,
        attrs: Attributes,
        title: Option<String>,
        summary: Option<String>,
        authors: Option<Vec<String>>,
    },
    Fragment {
        position: Option<Position>,
        children: Vec<Node>,
        attrs: Attributes,
    },
    Chapter {
        position: Option<Position>,
        children: Vec<Node>,
        attrs: Attributes,
    },
    Section {
        position: Option<Position>,
        children: Vec<Node>,
        attrs: Attributes,
    },
    Heading {
        position: Option<Position>,
        children: Vec<Node>,
        attrs: Attributes,
        level: u8,
    },
    BlockQuote {
        position: Option<Position>,
        children: Vec<Node>,
        attrs: Attributes,
    },
    LineBreak {
        position: Option<Position>,
    },
    SoftBreak {
        position: Option<Position>,
    },
    CodeBlock {
        position: Option<Position>,
        value: String,
        attrs: Attributes,
        info: String,
    },
    Definition {
        position: Option<Position>,
        children: Vec<Node>,
        attrs: Attributes,
        id: String,
        label: String,
        url: String,
        title: Option<String>,
    },
    Italic {
        position: Option<Position>,
        children: Vec<Node>,
        attrs: Attributes,
    },
    Html {
        position: Option<Position>,
        value: String,
        attrs: Attributes,
    },
    Image {
        position: Option<Position>,
        attrs: Attributes,
        url: String,
        alt: String,
        title: Option<String>,
    },
    ImageRef {
        position: Option<Position>,
        attrs: Attributes,
        id: String,
        label: String,
        alt: String,
    },
    InlineCode {
        position: Option<Position>,
        attrs: Attributes,
        value: String,
    },
    Link {
        position: Option<Position>,
        attrs: Attributes,
        children: Vec<Node>,
        url: String,
        title: String,
    },
    LinkRef {
        position: Option<Position>,
        attrs: Attributes,
        children: Vec<Node>,
        id: String,
        label: String,
    },
    List {
        position: Option<Position>,
        attrs: Attributes,
        children: Vec<Node>,
        ordered: bool,
        start: Option<usize>,
    },
    ListItem {
        position: Option<Position>,
        attrs: Attributes,
        children: Vec<Node>,
        checked: Option<bool>,
    },
    Paragraph {
        position: Option<Position>,
        attrs: Attributes,
        children: Vec<Node>,
    },
    Bold {
        position: Option<Position>,
        attrs: Attributes,
        children: Vec<Node>,
    },
    Superscript {
        position: Option<Position>,
        attrs: Attributes,
        children: Vec<Node>,
    },
    Text {
        position: Option<Position>,
        attrs: Attributes,
        value: String,
    },
    ThematicBreak {
        position: Option<Position>,
        attrs: Attributes,
    },
    StrikeThrough {
        position: Option<Position>,
        children: Vec<Node>,
        attrs: Attributes,
    },
    FootnoteDef {
        position: Option<Position>,
        attrs: Attributes,
        children: Vec<Node>,
        id: String,
    },
    FootnoteRef {
        position: Option<Position>,
        attrs: Attributes,
        id: String,
    },
    Table {
        position: Option<Position>,
        attrs: Attributes,
        children: Vec<Node>,
    },
    TableRow {
        position: Option<Position>,
        attrs: Attributes,
        children: Vec<Node>,
        is_header: bool,
    },
    TableCell {
        position: Option<Position>,
        attrs: Attributes,
        children: Vec<Node>,
    },
    Metadata {
        position: Option<Position>,
        attrs: Attributes,
        value: String,
    },
    DescrList {
        position: Option<Position>,
        attrs: Attributes,
        children: Vec<Node>,
    },
    DescrItem {
        position: Option<Position>,
        attrs: Attributes,
        children: Vec<Node>,
    },
    DescrTerm {
        position: Option<Position>,
        attrs: Attributes,
        children: Vec<Node>,
    },
    DescrDetail {
        position: Option<Position>,
        attrs: Attributes,
        children: Vec<Node>,
    },
    Comment {
        position: Option<Position>,
        attrs: Attributes,
        value: String,
    },
    Other {
        position: Option<Position>,
        attrs: Attributes,
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
}

/// Node attributes
pub type Attributes = HashMap<String, Option<String>>;

/// Node position
#[derive(Debug, Clone)]
pub struct Position {
    /// Start position
    pub start: Point,
    /// End position
    pub end: Point,
}

impl Position {
    /// Creates a new position
    pub fn new(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            start: Point::new(start_line, start_col),
            end: Point::new(end_line, end_col),
        }
    }
}

/// Node position point
#[derive(Debug, Clone)]
pub struct Point {
    /// Line
    pub line: usize,
    /// Column
    pub column: usize,
}

impl Point {
    /// Creates a new [Point]
    pub fn new(line: usize, column: usize) -> Point {
        Self { line, column }
    }
}
