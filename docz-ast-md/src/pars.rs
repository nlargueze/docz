//! Parsing

use comrak::{
    nodes::{AstNode, NodeValue},
    Arena, ComrakOptions,
};
use docz_ast::{Attributes, Error, Node, Parser, Point, Position};
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

        // node position
        let src_repos = &ast_root.data.borrow().sourcepos;
        let position = Some(Position {
            start: Point {
                line: src_repos.start.line,
                column: src_repos.start.column,
            },
            end: Point {
                line: src_repos.end.line,
                column: src_repos.end.column,
            },
        });

        // node attributes
        let attrs = Attributes::new();

        // node children
        let children = vec![];

        // parse node
        let mut node = match &ast_root.data.borrow().value {
            NodeValue::Document => Node::Document {
                position,
                children,
                attrs,
                title: None,
                summary: None,
                authors: None,
            },
            NodeValue::FrontMatter(fmatter) => Node::Metadata {
                position,
                attrs,
                value: fmatter.to_string(),
            },
            NodeValue::BlockQuote => Node::BlockQuote {
                position,
                children,
                attrs,
            },
            NodeValue::List(list) => match list.list_type {
                comrak::nodes::ListType::Bullet => Node::List {
                    position,
                    attrs,
                    children,
                    ordered: false,
                    start: None,
                },
                comrak::nodes::ListType::Ordered => Node::List {
                    position,
                    attrs,
                    children,
                    ordered: true,
                    start: Some(list.start),
                },
            },
            NodeValue::Item(_item) => Node::ListItem {
                position,
                attrs,
                children,
                checked: None,
            },
            NodeValue::DescriptionList => Node::DescrList {
                position,
                attrs,
                children,
            },
            NodeValue::DescriptionItem(_item) => Node::DescrItem {
                position,
                attrs,
                children,
            },
            NodeValue::DescriptionTerm => Node::DescrItem {
                position,
                attrs,
                children,
            },
            NodeValue::DescriptionDetails => Node::DescrDetail {
                position,
                attrs,
                children,
            },
            NodeValue::CodeBlock(block) => Node::CodeBlock {
                position,
                value: block.literal.to_string(),
                attrs,
                info: block.info.to_string(),
            },
            NodeValue::HtmlBlock(block) => Node::Html {
                position,
                attrs,
                value: block.literal.to_string(),
            },
            NodeValue::Paragraph => Node::Paragraph {
                position,
                attrs,
                children,
            },
            NodeValue::Heading(heading) => Node::Heading {
                position,
                children,
                attrs,
                level: heading.level,
            },
            NodeValue::ThematicBreak => Node::ThematicBreak { position, attrs },
            NodeValue::Table(_table) => Node::Table {
                position,
                attrs,
                children,
            },
            NodeValue::TableRow(row) => Node::TableRow {
                position,
                attrs,
                children,
                is_header: *row,
            },
            NodeValue::TableCell => Node::TableCell {
                position,
                attrs,
                children,
            },
            NodeValue::Text(text) => Node::Text {
                position,
                attrs,
                value: text.to_string(),
            },
            NodeValue::TaskItem(item) => Node::ListItem {
                position,
                attrs,
                children,
                checked: Some(item.is_some()),
            },
            NodeValue::SoftBreak => Node::SoftBreak { position },
            NodeValue::LineBreak => Node::LineBreak { position },
            NodeValue::Code(code) => Node::InlineCode {
                position,
                attrs,
                value: code.literal.to_string(),
            },
            NodeValue::HtmlInline(html) => Node::Html {
                position,
                value: html.to_string(),
                attrs,
            },
            NodeValue::Emph => Node::Italic {
                position,
                children,
                attrs,
            },
            NodeValue::Strong => Node::Bold {
                position,
                attrs,
                children,
            },
            NodeValue::Strikethrough => Node::StrikeThrough {
                position,
                attrs,
                children,
            },
            NodeValue::Superscript => Node::Superscript {
                position,
                attrs,
                children,
            },
            NodeValue::Link(link) => Node::Link {
                position,
                attrs,
                children,
                url: link.url.clone(),
                title: link.title.clone(),
            },
            NodeValue::Image(image) => Node::Image {
                position,
                attrs,
                url: image.url.clone(),
                alt: image.title.clone(),
                title: None,
            },
            NodeValue::FootnoteReference(footnote_ref) => Node::FootnoteRef {
                position,
                attrs,
                id: footnote_ref.to_string(),
            },
            NodeValue::FootnoteDefinition(footnote) => Node::FootnoteDef {
                position,
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
