//! Generic AST for docz

mod conv;
mod error;
mod json;

use serde::Serialize;

#[cfg(test)]
mod tests;

pub use conv::*;
pub use error::*;

/// AST node
#[derive(Debug, Clone, Serialize)]
pub struct Node<T>
where
    T: NodeData,
{
    /// Node data
    pub data: T,
    /// Span
    pub span: Option<Span>,
    /// Children
    pub children: Vec<Node<T>>,
}

/// Marker trait for the type of AST node
pub trait NodeData {}

impl<T> Node<T>
where
    T: NodeData,
{
    /// Creates a new node
    pub fn new(data: T) -> Self {
        Self {
            data,
            span: None,
            children: vec![],
        }
    }

    /// Adds a span
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    /// Adds a child
    pub fn with_child(mut self, child: Node<T>) -> Self {
        self.children.push(child);
        self
    }

    /// Adds children
    pub fn with_children(mut self, children: Vec<Node<T>>) -> Self {
        self.children = children;
        self
    }

    /// Visits a node recursively
    pub fn visit(&self, f: &mut impl FnMut(&Node<T>)) {
        f(self);
        for child in self.children.iter() {
            child.visit(f);
        }
    }

    /// Visits a node recursively mutably
    pub fn visit_mut(&mut self, f: &mut impl FnMut(&mut Node<T>)) {
        f(self);
        for child in self.children.iter_mut() {
            child.visit_mut(f);
        }
    }

    /// Returns the number of nodes
    pub fn nb_nodes(&self) -> usize {
        let mut nb_nodes = 0;
        self.visit(&mut |_node| {
            nb_nodes += 1;
        });
        nb_nodes
    }
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
