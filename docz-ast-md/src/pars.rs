//! Parsing

use comrak::{
    nodes::{AstNode, NodeValue},
    Arena, ComrakOptions,
};
use docz_ast::{Attrs, Error, Node, Parser, Span};
use serde::Deserialize;

/// AST parser for markdown
#[derive(Debug, Default)]
pub struct MdParser {}

impl MdParser {
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }
}

impl Parser for MdParser {
    fn parse(&self, data: &str) -> Result<Node, Error> {
        let arena = Arena::new();
        let mut opts = ComrakOptions::default();
        opts.extension.strikethrough = true;
        opts.extension.tagfilter = true;
        opts.extension.table = true;
        opts.extension.autolink = true;
        opts.extension.tasklist = true;
        opts.extension.footnotes = true;
        opts.extension.front_matter_delimiter = Some("---".to_string());
        let md_root = comrak::parse_document(&arena, data, &opts);
        // eprintln!("{md_root:#?}");

        self.parse_node_iter(md_root)
    }
}

impl MdParser {
    /// Parses a node recursively
    #[allow(clippy::only_used_in_recursion)]
    fn parse_node_iter<'a>(&self, ast_root: &'a AstNode<'a>) -> Result<Node, Error> {
        // eprintln!("ast_root: {:#?}", ast_root.data);

        // node span
        let src_repos = &ast_root.data.borrow().sourcepos;
        let span = Some(Span::new(
            src_repos.start.line,
            src_repos.start.column,
            src_repos.end.line,
            src_repos.end.column,
        ));

        // node attributes
        let attrs = Attrs::new();

        // node children
        let children = vec![];

        // parse node
        let mut node = match &ast_root.data.borrow().value {
            NodeValue::Document => Node::Document {
                span,
                children,
                attrs,
                title: None,
                summary: None,
                authors: None,
            },
            NodeValue::FrontMatter(fmatter) => Node::Metadata {
                span,
                attrs,
                value: fmatter.to_string(),
            },
            NodeValue::BlockQuote => Node::BlockQuote {
                span,
                children,
                attrs,
            },
            NodeValue::List(list) => match list.list_type {
                comrak::nodes::ListType::Bullet => Node::List {
                    span,
                    attrs,
                    children,
                    ordered: false,
                    start: None,
                },
                comrak::nodes::ListType::Ordered => Node::List {
                    span,
                    attrs,
                    children,
                    ordered: true,
                    start: Some(list.start),
                },
            },
            NodeValue::Item(_item) => Node::ListItem {
                span,
                attrs,
                children,
                checked: None,
            },
            NodeValue::DescriptionList => Node::DescrList {
                span,
                attrs,
                children,
            },
            NodeValue::DescriptionItem(_item) => Node::DescrItem {
                span,
                attrs,
                children,
            },
            NodeValue::DescriptionTerm => Node::DescrItem {
                span,
                attrs,
                children,
            },
            NodeValue::DescriptionDetails => Node::DescrDetail {
                span,
                attrs,
                children,
            },
            NodeValue::CodeBlock(block) => Node::CodeBlock {
                span,
                value: block.literal.to_string(),
                attrs,
                info: block.info.to_string(),
            },
            NodeValue::HtmlBlock(block) => Node::Html {
                span,
                attrs,
                value: block.literal.to_string(),
            },
            NodeValue::Paragraph => Node::Paragraph {
                span,
                attrs,
                children,
            },
            NodeValue::Heading(heading) => Node::Heading {
                span,
                children,
                attrs,
                level: heading.level,
            },
            NodeValue::ThematicBreak => Node::ThematicBreak { span, attrs },
            NodeValue::Table(_table) => Node::Table {
                span,
                attrs,
                children,
            },
            NodeValue::TableRow(row) => Node::TableRow {
                span,
                attrs,
                children,
                is_header: *row,
            },
            NodeValue::TableCell => Node::TableCell {
                span,
                attrs,
                children,
            },
            NodeValue::Text(text) => Node::Text {
                span,
                attrs,
                value: text.to_string(),
            },
            NodeValue::TaskItem(item) => Node::ListItem {
                span,
                attrs,
                children,
                checked: Some(item.is_some()),
            },
            NodeValue::SoftBreak => Node::SoftBreak { span },
            NodeValue::LineBreak => Node::LineBreak { span },
            NodeValue::Code(code) => Node::InlineCode {
                span,
                attrs,
                value: code.literal.to_string(),
            },
            NodeValue::HtmlInline(html) => Node::Html {
                span,
                value: html.to_string(),
                attrs,
            },
            NodeValue::Emph => Node::Italic {
                span,
                children,
                attrs,
            },
            NodeValue::Strong => Node::Bold {
                span,
                attrs,
                children,
            },
            NodeValue::Strikethrough => Node::StrikeThrough {
                span,
                attrs,
                children,
            },
            NodeValue::Superscript => Node::Superscript {
                span,
                attrs,
                children,
            },
            NodeValue::Link(link) => Node::Link {
                span,
                attrs,
                children,
                url: link.url.clone(),
                title: link.title.clone(),
            },
            NodeValue::Image(image) => Node::Image {
                span,
                attrs,
                url: image.url.clone(),
                alt: image.title.clone(),
                title: None,
            },
            NodeValue::FootnoteReference(footnote_ref) => Node::FootnoteRef {
                span,
                attrs,
                id: footnote_ref.to_string(),
            },
            NodeValue::FootnoteDefinition(footnote) => Node::FootnoteDef {
                span,
                attrs,
                children,
                id: footnote.to_string(),
            },
        };

        if let Some(children) = node.children_mut() {
            for ast_child in ast_root.children() {
                let child = self.parse_node_iter(ast_child)?;
                children.push(child);
            }
        }

        Ok(node)
    }
}

/// Parses the frontmatter YAML string
pub fn parse_frontmatter<'a, T>(value: &'a str) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    serde_yaml::from_str::<T>(value).map_err(|e| Error::new(&e.to_string()))
}
