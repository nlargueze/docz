//! Parser

use docz_ast::{Error, Node, Parser, Span};
use html_parser::{Dom, ElementVariant};

use crate::Html;

/// HTML parser
#[derive(Debug, Default)]
pub struct HTMLParser {}

impl HTMLParser {
    /// Creates a new parser
    pub fn new() -> Self {
        Self::default()
    }
}

impl Parser<Html> for HTMLParser {
    fn parse(&self, data: &[u8]) -> Result<Node<Html>, Error> {
        let data_str =
            String::from_utf8(data.to_vec()).map_err(|e| Error::new(e.to_string().as_str()))?;
        let dom = Dom::parse(&data_str).map_err(|e| Error::new(e.to_string().as_str()))?;

        if let Some(err) = dom.errors.into_iter().next() {
            return Err(Error::new(err.as_str()));
        }

        let mut children = vec![];
        for html_child in &dom.children {
            let child_node = self.parse_node_iter(html_child)?;
            children.push(child_node);
        }

        let node = Node::new(Html::Fragment).with_children(children);
        Ok(node)
    }
}

impl HTMLParser {
    /// Parses a node in a recursive manner
    #[allow(clippy::only_used_in_recursion)]
    fn parse_node_iter(&self, html_node: &html_parser::Node) -> Result<Node<Html>, Error> {
        let node = match html_node {
            html_parser::Node::Text(text) => Node::new(Html::Text {
                value: text.clone(),
            }),
            html_parser::Node::Comment(comment) => Node::new(Html::Comment {
                value: comment.clone(),
            }),
            html_parser::Node::Element(element) => {
                let mut node = self.parse_html_element(element);
                for child in element.children.iter() {
                    let child = self.parse_node_iter(child)?;
                    node.children.push(child);
                }
                node
            }
        };

        Ok(node)
    }

    /// Maps HTML tags to AST types
    fn parse_html_element(&self, element: &html_parser::Element) -> Node<Html> {
        // tag + attributes
        let tag = element.name.clone();
        let self_closing = element.variant == ElementVariant::Void;
        let id = element.id.clone();
        let attrs = element.attributes.clone();
        let classes = element.classes.clone();

        // span
        let span = Span::new(
            element.source_span.start_line,
            element.source_span.start_column,
            element.source_span.end_line,
            element.source_span.end_column,
        );

        Node::new(Html::Element {
            tag,
            void: self_closing,
            id,
            attrs,
            classes,
        })
        .with_span(span)
    }
}
