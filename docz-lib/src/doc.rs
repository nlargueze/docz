//! Document

use std::collections::HashMap;

use crate::ast::Node;

/// A document
#[derive(Debug, Default)]
pub struct Document {
    /// Medatada
    pub metadata: HashMap<String, String>,
    /// Fragments
    pub fragments: Vec<Fragment>,
}

impl Document {
    /// Adds a fragment to the document
    pub fn add_fragment(&mut self, fragment: Fragment) {
        self.fragments.push(fragment);
    }
}

/// A document fragment
#[derive(Debug, Default)]
pub struct Fragment {
    /// Medatada
    pub metadata: HashMap<String, String>,
    /// AST root node
    pub root: Node,
}

impl Fragment {
    /// Adds a child node to the root node
    pub fn add_child(&mut self, node: Node) {
        self.root.children.push(node);
    }
}
