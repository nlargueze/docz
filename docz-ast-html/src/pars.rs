//! Parser

use std::collections::HashMap;

use docz_ast::{AstParser, Error, Node, NodeType};
use html_parser::Dom;

/// HTML parser
#[derive(Debug, Default)]
pub struct HTMLParser {}

impl HTMLParser {
    /// Creates a new parser
    pub fn new() -> Self {
        Self::default()
    }
}

impl AstParser for HTMLParser {
    fn parse(&self, data: &str) -> Result<Node, Error> {
        let dom = Dom::parse(data).map_err(|e| Error::new(e.to_string().as_str()))?;

        if let Some(err) = dom.errors.into_iter().next() {
            return Err(Error::new(err.as_str()));
        }

        let mut node = Node::default();
        for html_child in &dom.children {
            let child_node = self.parse_node_iter(html_child)?;
            node.add_child(child_node);
        }

        Ok(node)
    }
}

impl HTMLParser {
    /// Parses a node in a recursive manner
    #[allow(clippy::only_used_in_recursion)]
    fn parse_node_iter(&self, html_node: &html_parser::Node) -> Result<Node, Error> {
        let mut node = Node::default();

        match html_node {
            html_parser::Node::Text(text) => {
                node.set_type(NodeType::Text).set_value(text);
            }
            html_parser::Node::Comment(comment) => {
                node.set_type(NodeType::Comment).set_value(comment);
            }
            html_parser::Node::Element(element) => {
                let ty = Self::map_html_tag_to_ast_type(&element.name, &element.attributes);
                node.set_type(ty);

                if let Some(id) = element.id.as_ref() {
                    node.add_attr("id", Some(id.as_str()));
                }
                for (key, value) in element.attributes.iter() {
                    node.add_attr(key, value.as_deref());
                }
                // NB: add classes
                for child in element.children.iter() {
                    let child = self.parse_node_iter(child)?;
                    node.add_child(child);
                }
            }
        }

        Ok(node)
    }

    /// Maps HTML tags to AST types
    fn map_html_tag_to_ast_type(tag: &str, _attrs: &HashMap<String, Option<String>>) -> NodeType {
        match tag {
            "a" => todo!(),
            "abbr" => todo!(),
            "address" => todo!(),
            "area" => todo!(),
            "article" => todo!(),
            "aside" => todo!(),
            "audio" => todo!(),
            "b" => NodeType::Bold,
            "base" => todo!(),
            "bdi" => todo!(),
            "bdo" => todo!(),
            "blockquote" => NodeType::BlockQuote,
            "body" => todo!(),
            "br" => NodeType::LineBreak,
            "button" => todo!(),
            "canvas" => todo!(),
            "caption" => todo!(),
            "cite" => NodeType::BlockQuote,
            "code" => NodeType::Code,
            "col" => todo!(),
            "colgroup" => todo!(),
            "data" => todo!(),
            "datalist" => todo!(),
            "dd" => NodeType::DescrDetails,
            "del" => todo!(),
            "details" => todo!(),
            "dfn" => todo!(),
            "dialog" => todo!(),
            "div" => todo!(),
            "dl" => NodeType::DescrList,
            "dt" => NodeType::DescrTerm,
            "em" => NodeType::Italic,
            "embed" => todo!(),
            "fieldset" => todo!(),
            "figcaption" => todo!(),
            "figure" => todo!(),
            "footer" => todo!(),
            "form" => todo!(),
            "head" => todo!(),
            "header" => todo!(),
            "hgroup" => todo!(),
            "h1" => todo!(),
            "h2" => todo!(),
            "h3" => todo!(),
            "h4" => todo!(),
            "h5" => todo!(),
            "h6" => todo!(),
            "hr" => todo!(),
            "html" => todo!(),
            "i" => todo!(),
            "iframe" => todo!(),
            "img" => todo!(),
            "input" => todo!(),
            "ins" => todo!(),
            "kbd" => todo!(),
            "keygen" => todo!(),
            "label" => todo!(),
            "legend" => todo!(),
            "li" => todo!(),
            "link" => todo!(),
            "main" => todo!(),
            "map" => todo!(),
            "mark" => todo!(),
            "menu" => todo!(),
            "menuitem" => todo!(),
            "meta" => todo!(),
            "meter" => todo!(),
            "nav" => todo!(),
            "noscript" => todo!(),
            "object" => todo!(),
            "ol" => todo!(),
            "optgroup" => todo!(),
            "option" => todo!(),
            "output" => todo!(),
            "p" => todo!(),
            "param" => todo!(),
            "pre" => todo!(),
            "progress" => todo!(),
            "q" => todo!(),
            "rp" => todo!(),
            "rt" => todo!(),
            "ruby" => todo!(),
            "s" => todo!(),
            "samp" => todo!(),
            "script" => todo!(),
            "section" => todo!(),
            "select" => todo!(),
            "small" => todo!(),
            "source" => todo!(),
            "span" => todo!(),
            "strong" => todo!(),
            "style" => todo!(),
            "sub" => todo!(),
            "summary" => todo!(),
            "sup" => todo!(),
            "svg" => todo!(),
            "table" => todo!(),
            "tbody" => todo!(),
            "td" => todo!(),
            "template" => todo!(),
            "textarea" => todo!(),
            "tfoot" => todo!(),
            "th" => todo!(),
            "thead" => todo!(),
            "time" => todo!(),
            "title" => todo!(),
            "tr" => todo!(),
            "track" => todo!(),
            "u" => todo!(),
            "ul" => todo!(),
            "var" => todo!(),
            "video" => todo!(),
            "wbr" => todo!(),
            _ => todo!(),
        }
    }
}
