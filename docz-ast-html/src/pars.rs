//! Parser

use docz_ast::{Attrs, Error, Node, Parser, Span};
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

impl Parser for HTMLParser {
    fn parse(&self, data: &str) -> Result<Node, Error> {
        let dom = Dom::parse(data).map_err(|e| Error::new(e.to_string().as_str()))?;

        if let Some(err) = dom.errors.into_iter().next() {
            return Err(Error::new(err.as_str()));
        }

        let mut children = vec![];
        for html_child in &dom.children {
            let child_node = self.parse_node_iter(html_child)?;
            children.push(child_node);
        }

        Ok(Node::Fragment {
            span: None,
            children,
            attrs: Attrs::new(),
        })
    }
}

impl HTMLParser {
    /// Parses a node in a recursive manner
    #[allow(clippy::only_used_in_recursion)]
    fn parse_node_iter(&self, html_node: &html_parser::Node) -> Result<Node, Error> {
        let node = match html_node {
            html_parser::Node::Text(text) => Node::Text {
                span: None,
                attrs: Attrs::default(),
                value: text.clone(),
            },
            html_parser::Node::Comment(comment) => Node::Comment {
                span: None,
                attrs: Attrs::default(),
                value: comment.clone(),
            },
            html_parser::Node::Element(element) => {
                let mut node = self.parse_html_element(element);
                if let Some(children) = node.children_mut() {
                    for child in element.children.iter() {
                        let child = self.parse_node_iter(child)?;
                        children.push(child);
                    }
                }
                node
            }
        };

        Ok(node)
    }

    /// Maps HTML tags to AST types
    fn parse_html_element(&self, element: &html_parser::Element) -> Node {
        // node span
        let span = Some(Span::new(
            element.source_span.start_line,
            element.source_span.start_column,
            element.source_span.end_line,
            element.source_span.end_column,
        ));

        // node attributes
        let attrs = Attrs::default();

        // node children
        let children = vec![];

        // node
        let node = match element.name.as_str() {
            "a" => todo!(),
            "abbr" => todo!(),
            "address" => todo!(),
            "area" => todo!(),
            "article" => todo!(),
            "aside" => todo!(),
            "audio" => todo!(),
            "b" => Node::Bold {
                span,
                attrs,
                children,
            },
            "base" => todo!(),
            "bdi" => todo!(),
            "bdo" => todo!(),
            "blockquote" => Node::BlockQuote {
                span,
                children,
                attrs,
            },
            "body" => todo!(),
            "br" => Node::LineBreak { span },
            "button" => todo!(),
            "canvas" => todo!(),
            "caption" => todo!(),
            "cite" => Node::BlockQuote {
                span,
                children,
                attrs,
            },
            "code" => Node::InlineCode {
                span,
                value: "".to_string(),
                attrs,
            },
            "col" => todo!(),
            "colgroup" => todo!(),
            "data" => todo!(),
            "datalist" => todo!(),
            "dd" => todo!(), // NodeType::DescrDetails,
            "del" => todo!(),
            "details" => todo!(),
            "dfn" => todo!(),
            "dialog" => todo!(),
            "div" => todo!(),
            "dl" => todo!(), // NodeType::DescrList,
            "dt" => todo!(), // NodeType::DescrTerm,
            "em" => todo!(), // NodeType::Italic,
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
        };

        // if let Some(id) = element.id.as_ref() {
        //     node.add_attr("id", Some(id.as_str()));
        // }
        // for (key, value) in element.attributes.iter() {
        //     node.add_attr(key, value.as_deref());
        // }
        // NB: add classes

        node
    }
}
