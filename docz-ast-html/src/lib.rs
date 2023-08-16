//! HTML AST for docz

mod ast;
mod error;
mod parse;
mod render;

use std::collections::HashMap;

pub use ast::*;
pub use error::*;
pub use parse::*;
pub use render::*;

#[cfg(test)]
mod tests;

/// HTML node
#[derive(Debug, Clone)]
pub enum HtmlNode {
    Document {
        children: Vec<HtmlNode>,
    },
    Fragment {
        children: Vec<HtmlNode>,
    },
    Text {
        value: String,
    },
    Comment {
        value: String,
    },
    Element {
        tag: String,
        void: bool,
        id: Option<String>,
        attrs: HashMap<String, Option<String>>,
        classes: Vec<String>,
        children: Vec<HtmlNode>,
    },
}
