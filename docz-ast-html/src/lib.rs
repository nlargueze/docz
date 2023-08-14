//! HTML AST for docz

mod pars;
mod rend;

use std::collections::HashMap;

use docz_ast::NodeData;
pub use pars::*;
pub use rend::*;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum Html {
    Fragment,
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
    },
}

impl NodeData for Html {}
