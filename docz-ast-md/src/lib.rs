//! Markdown AST for docz.

mod fmatter;
mod pars;
mod rend;

use docz_ast::NodeData;
pub use fmatter::*;
pub use pars::*;
pub use rend::*;
use serde::Serialize;

#[cfg(test)]
mod tests;

/// Markdown node data
#[derive(Debug, Clone, Serialize)]
pub enum Markdown {
    Document,
    Fragment,
    FrontMatter { value: String },
    Chapter,
    Section,
    Heading { level: u8, id: Option<String> },
    Paragraph,
    Text { value: String },
    Comment { value: String },
    ThematicBreak,
    LineBreak,
    SoftBreak,
    Italic,
    Bold,
    BlockQuote,
    List { ordered: bool },
    ListItem { index: Option<usize> },
    CodeInline { value: String },
    CodeBlock { info: String, value: String },
    Link { url: String, title: Option<String> },
    Image { url: String, title: Option<String> },
    HtmlInline { value: String },
    HtmlBlock { value: String },
    Table { columns: Vec<MdTableAlignment> },
    TableRow { is_header: bool },
    TableCell,
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

impl NodeData for Markdown {}

/// Table Alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum MdTableAlignment {
    None,
    Left,
    Center,
    Right,
}
