//! Markdown AST for docz.

mod ast;
mod error;
mod parse;
mod render;

pub use ast::*;
pub use error::*;
pub use parse::*;
pub use render::*;

use serde::Serialize;

#[cfg(test)]
mod tests;

/// Markdown node
#[derive(Debug, Clone, Serialize)]
pub enum MdNode {
    Document {
        children: Vec<MdNode>,
    },
    FrontMatter {
        value: String,
    },
    Heading {
        level: u8,
        id: Option<String>,
        children: Vec<MdNode>,
    },
    Paragraph {
        children: Vec<MdNode>,
    },
    Text {
        value: String,
    },
    Comment {
        value: String,
    },
    ThematicBreak,
    LineBreak,
    SoftBreak,
    Italic {
        children: Vec<MdNode>,
    },
    Bold {
        children: Vec<MdNode>,
    },
    BlockQuote {
        children: Vec<MdNode>,
    },
    List {
        ordered: bool,
        children: Vec<MdNode>,
    },
    ListItem {
        index: Option<usize>,
        children: Vec<MdNode>,
    },
    CodeInline {
        value: String,
    },
    CodeBlock {
        info: String,
        value: String,
    },
    Link {
        url: String,
        title: Option<String>,
    },
    Image {
        url: String,
        title: Option<String>,
    },
    HtmlInline {
        value: String,
    },
    HtmlBlock {
        value: String,
    },
    Table {
        columns: Vec<MdTableAlignment>,
        children: Vec<MdNode>,
    },
    TableRow {
        is_header: bool,
        children: Vec<MdNode>,
    },
    TableCell {
        children: Vec<MdNode>,
    },
    FootnoteRef {
        id: String,
    },
    FootnoteDef {
        id: String,
    },
    DefinitionList {
        children: Vec<MdNode>,
    },
    DefinitionItem {
        children: Vec<MdNode>,
    },
    DefinitionTerm {
        children: Vec<MdNode>,
    },
    DefinitionDetails {
        children: Vec<MdNode>,
    },
    StrikeThrough {
        children: Vec<MdNode>,
    },
    TaskItem {
        checked: bool,
        children: Vec<MdNode>,
    },
    Highlight {
        children: Vec<MdNode>,
    },
    SubScript {
        children: Vec<MdNode>,
    },
    SuperScript {
        children: Vec<MdNode>,
    },
}

/// Front matter separator
pub const FRONTMATTER_SEP: &str = "---";

/// Table Alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum MdTableAlignment {
    None,
    Left,
    Center,
    Right,
}
