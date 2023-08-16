//! AST

use docz_ast::AstNode;

use crate::MdNode;

impl AstNode for MdNode {
    fn children(&self) -> Option<&Vec<Self>> {
        match self {
            MdNode::Document { children } => Some(children),
            MdNode::FrontMatter { .. } => None,
            MdNode::Heading { children, .. } => Some(children),
            MdNode::Paragraph { children } => Some(children),
            MdNode::Text { .. } => None,
            MdNode::Comment { .. } => None,
            MdNode::ThematicBreak => None,
            MdNode::LineBreak => None,
            MdNode::SoftBreak => None,
            MdNode::Italic { children } => Some(children),
            MdNode::Bold { children } => Some(children),
            MdNode::BlockQuote { children } => Some(children),
            MdNode::List { children, .. } => Some(children),
            MdNode::ListItem { children, .. } => Some(children),
            MdNode::CodeInline { .. } => None,
            MdNode::CodeBlock { .. } => None,
            MdNode::Link { .. } => None,
            MdNode::Image { .. } => None,
            MdNode::HtmlInline { .. } => None,
            MdNode::HtmlBlock { .. } => None,
            MdNode::Table { children, .. } => Some(children),
            MdNode::TableRow { children, .. } => Some(children),
            MdNode::TableCell { children } => Some(children),
            MdNode::FootnoteRef { .. } => None,
            MdNode::FootnoteDef { .. } => None,
            MdNode::DefinitionList { children } => Some(children),
            MdNode::DefinitionItem { children } => Some(children),
            MdNode::DefinitionTerm { children } => Some(children),
            MdNode::DefinitionDetails { children } => Some(children),
            MdNode::StrikeThrough { children } => Some(children),
            MdNode::TaskItem { children, .. } => Some(children),
            MdNode::Highlight { children } => Some(children),
            MdNode::SubScript { children } => Some(children),
            MdNode::SuperScript { children } => Some(children),
        }
    }

    fn children_mut(&mut self) -> Option<&mut Vec<Self>> {
        match self {
            MdNode::Document { children } => Some(children),
            MdNode::FrontMatter { .. } => None,
            MdNode::Heading { children, .. } => Some(children),
            MdNode::Paragraph { children } => Some(children),
            MdNode::Text { .. } => None,
            MdNode::Comment { .. } => None,
            MdNode::ThematicBreak => None,
            MdNode::LineBreak => None,
            MdNode::SoftBreak => None,
            MdNode::Italic { children } => Some(children),
            MdNode::Bold { children } => Some(children),
            MdNode::BlockQuote { children } => Some(children),
            MdNode::List { children, .. } => Some(children),
            MdNode::ListItem { children, .. } => Some(children),
            MdNode::CodeInline { .. } => None,
            MdNode::CodeBlock { .. } => None,
            MdNode::Link { .. } => None,
            MdNode::Image { .. } => None,
            MdNode::HtmlInline { .. } => None,
            MdNode::HtmlBlock { .. } => None,
            MdNode::Table { children, .. } => Some(children),
            MdNode::TableRow { children, .. } => Some(children),
            MdNode::TableCell { children } => Some(children),
            MdNode::FootnoteRef { .. } => None,
            MdNode::FootnoteDef { .. } => None,
            MdNode::DefinitionList { children } => Some(children),
            MdNode::DefinitionItem { children } => Some(children),
            MdNode::DefinitionTerm { children } => Some(children),
            MdNode::DefinitionDetails { children } => Some(children),
            MdNode::StrikeThrough { children } => Some(children),
            MdNode::TaskItem { children, .. } => Some(children),
            MdNode::Highlight { children } => Some(children),
            MdNode::SubScript { children } => Some(children),
            MdNode::SuperScript { children } => Some(children),
        }
    }
}
