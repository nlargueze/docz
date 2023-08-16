//! Parser

use html_parser::{Dom, DomVariant, ElementVariant};

use crate::{Error, HtmlNode};

/// HTML parser
#[derive(Debug, Clone, Default)]
pub struct ParseOptions {}

impl HtmlNode {
    /// Parses to an HTML node
    pub fn parse(input: &str, opts: ParseOptions) -> Result<Self, Error> {
        let dom = Dom::parse(input)?;

        if let Some(err) = dom.errors.into_iter().next() {
            return Err(Error::new(err.as_str()));
        }

        let mut children = vec![];
        for child in dom.children {
            children.push(parse_node_iter(child, &opts)?);
        }

        let node = match dom.tree_type {
            DomVariant::Document => HtmlNode::Document { children },
            DomVariant::DocumentFragment => HtmlNode::Fragment { children },
            DomVariant::Empty => {
                return Err(Error::new("Empty HTML string"));
            }
        };

        Ok(node)
    }
}

/// Parses a node recursively
#[allow(clippy::only_used_in_recursion)]
fn parse_node_iter(node: html_parser::Node, opts: &ParseOptions) -> Result<HtmlNode, Error> {
    let html_node = match node {
        html_parser::Node::Text(text) => HtmlNode::Text {
            value: text.clone(),
        },
        html_parser::Node::Comment(comment) => HtmlNode::Comment {
            value: comment.clone(),
        },
        html_parser::Node::Element(elt) => {
            let mut children = vec![];
            for child in elt.children {
                children.push(parse_node_iter(child, opts)?);
            }

            HtmlNode::Element {
                tag: elt.name.clone(),
                void: elt.variant == ElementVariant::Void,
                id: elt.id.clone(),
                attrs: elt.attributes.clone(),
                classes: elt.classes.clone(),
                children,
            }
        }
    };

    Ok(html_node)
}
