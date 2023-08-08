//! HTML AST for docz

use docz_ast::{Node, NodeType, Parser, Renderer};
use error::Error;
use html_parser::Dom;

pub mod error;
#[cfg(test)]
mod tests;

/// AST converter for HTML
#[derive(Debug, Default)]
pub struct HtmlConverter {}

impl HtmlConverter {
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }
}

impl Parser for HtmlConverter {
    type Err = Error;

    fn parse(&self, data: &str) -> Result<Node, Self::Err> {
        let dom = Dom::parse(data).map_err(|e| Error::Invalid(e.to_string()))?;

        if let Some(err) = dom.errors.into_iter().next() {
            return Err(Error::Invalid(err));
        }

        let mut node = Node::default();
        for html_child in &dom.children {
            let child_node = self.parse_node_iter(html_child)?;
            node.child(child_node);
        }

        Ok(node)
    }
}

impl HtmlConverter {
    /// Parses a node in a recursive manner
    #[allow(clippy::only_used_in_recursion)]
    fn parse_node_iter(&self, html_node: &html_parser::Node) -> Result<Node, Error> {
        let mut node = Node::default();

        match html_node {
            html_parser::Node::Text(text) => {
                node.ty(NodeType::Text);
                node.content(text);
            }
            html_parser::Node::Comment(comment) => {
                node.ty(NodeType::Comment);
                node.content(comment);
            }
            html_parser::Node::Element(element) => {
                node.tag(element.name.as_str());
                if let Some(id) = element.id.as_ref() {
                    node.attr("id", id.as_str());
                }
                // TODO: add attributes and classes

                for child in element.children.iter() {
                    let child = self.parse_node_iter(child)?;
                    node.child(child);
                }
            }
        }

        Ok(node)
    }
}

impl Renderer for HtmlConverter {
    type Err = Error;

    fn render(&self, node: &Node) -> Result<String, Self::Err> {
        let mut data = String::new();

        todo!("render");

        Ok(data)
    }
}
