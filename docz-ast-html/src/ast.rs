//! AST

use docz_ast::AstNode;

use crate::HtmlNode;

impl AstNode for HtmlNode {
    fn children(&self) -> Option<&Vec<Self>> {
        match self {
            HtmlNode::Document { children } => Some(children),
            HtmlNode::Fragment { children } => Some(children),
            HtmlNode::Text { .. } => None,
            HtmlNode::Comment { .. } => None,
            HtmlNode::Element { children, .. } => Some(children),
        }
    }

    fn children_mut(&mut self) -> Option<&mut Vec<Self>> {
        match self {
            HtmlNode::Document { children } => Some(children),
            HtmlNode::Fragment { children } => Some(children),
            HtmlNode::Text { .. } => None,
            HtmlNode::Comment { .. } => None,
            HtmlNode::Element { children, .. } => Some(children),
        }
    }
}
