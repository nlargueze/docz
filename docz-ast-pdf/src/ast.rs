//! AST

use docz_ast::AstNode;

use crate::PdfNode;

impl AstNode for PdfNode {
    fn children(&self) -> Option<&Vec<Self>> {
        match self {
            PdfNode::Document { children, .. } => Some(children),
            PdfNode::Section { children, .. } => Some(children),
            PdfNode::Paragraph { children } => Some(children),
            PdfNode::Text { .. } => None,
            PdfNode::Image { .. } => None,
            PdfNode::Formula { .. } => None,
        }
    }

    fn children_mut(&mut self) -> Option<&mut Vec<Self>> {
        match self {
            PdfNode::Document { children, .. } => Some(children),
            PdfNode::Section { children, .. } => Some(children),
            PdfNode::Paragraph { children } => Some(children),
            PdfNode::Text { .. } => None,
            PdfNode::Image { .. } => None,
            PdfNode::Formula { .. } => None,
        }
    }
}
