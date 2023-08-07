//! Document

use crate::ast::Ast;

/// A document
#[derive(Debug, Default)]
pub struct Document {
    /// Fragments
    pub fragments: Vec<Fragment>,
}

impl Document {
    /// Instantiates a new [Document]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a fragment to the document
    pub fn add_fragment(&mut self, fragment: Fragment) {
        self.fragments.push(fragment);
    }
}

/// A document fragment
#[derive(Debug)]
pub struct Fragment {
    /// AST root node
    pub ast: Ast,
}
