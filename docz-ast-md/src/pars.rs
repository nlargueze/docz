//! Parsing

use comrak::{
    nodes::{AstNode, NodeValue},
    Arena, ComrakExtensionOptions, ComrakOptions, ComrakParseOptions, ComrakRenderOptions,
};
use docz_ast::{Error, Node, Parser, Span};

use crate::{Markdown, MdTableAlignment, FRONTMATTER_SEP};

/// AST parser for markdown
#[derive(Debug, Default)]
pub struct MdParser {}

impl MdParser {
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }
}

impl Parser<Markdown> for MdParser {
    fn parse(&self, data: &[u8]) -> Result<Node<Markdown>, Error> {
        let arena = Arena::new();
        let opts = ComrakOptions {
            extension: ComrakExtensionOptions {
                strikethrough: true,
                tagfilter: true,
                table: true,
                autolink: true,
                tasklist: true,
                superscript: true,
                header_ids: Some("user-content-".to_string()),
                footnotes: true,
                description_lists: true,
                front_matter_delimiter: Some(FRONTMATTER_SEP.to_string()),
            },
            parse: ComrakParseOptions {
                smart: true,
                default_info_string: None,
                relaxed_tasklist_matching: true,
            },
            render: ComrakRenderOptions::default(),
        };
        let buffer = String::from_utf8_lossy(data);
        let md_root = comrak::parse_document(&arena, &buffer, &opts);
        // eprintln!("{md_root:#?}");

        self.parse_node_iter(md_root)
    }
}

impl MdParser {
    /// Parses a node recursively
    #[allow(clippy::only_used_in_recursion)]
    fn parse_node_iter<'a>(&self, ast_root: &'a AstNode<'a>) -> Result<Node<Markdown>, Error> {
        // data
        let data = match &ast_root.data.borrow().value {
            NodeValue::Document => Markdown::Document,
            NodeValue::FrontMatter(fmatter) => {
                let fmatter = fmatter.trim();
                let fmatter = fmatter
                    .strip_prefix(FRONTMATTER_SEP)
                    .ok_or(Error::new("invalid front matter prefix"))?;
                let fmatter = fmatter
                    .strip_suffix(FRONTMATTER_SEP)
                    .ok_or(Error::new("invalid front matter suffix"))?;
                let value = fmatter.trim();
                Markdown::FrontMatter {
                    value: value.to_string(),
                }
            }
            NodeValue::BlockQuote => Markdown::BlockQuote,
            NodeValue::List(list) => match list.list_type {
                comrak::nodes::ListType::Bullet => Markdown::List { ordered: false },
                comrak::nodes::ListType::Ordered => Markdown::List { ordered: true },
            },
            NodeValue::Item(item) => Markdown::ListItem {
                index: if item.list_type == comrak::nodes::ListType::Ordered {
                    Some(item.start)
                } else {
                    None
                },
            },
            NodeValue::DescriptionList => Markdown::DefinitionList,
            NodeValue::DescriptionItem(_item) => Markdown::DefinitionItem,
            NodeValue::DescriptionTerm => Markdown::DefinitionTerm,
            NodeValue::DescriptionDetails => Markdown::DefinitionDetails,
            NodeValue::CodeBlock(block) => Markdown::CodeBlock {
                info: block.info.clone(),
                value: block.literal.clone(),
            },
            NodeValue::HtmlBlock(block) => Markdown::HtmlBlock {
                value: block.literal.clone(),
            },
            NodeValue::Paragraph => Markdown::Paragraph,
            NodeValue::Heading(heading) => Markdown::Heading {
                level: heading.level,
                id: None,
            },
            NodeValue::ThematicBreak => Markdown::ThematicBreak,
            NodeValue::Table(table) => Markdown::Table {
                columns: table
                    .iter()
                    .map(|align| match align {
                        comrak::nodes::TableAlignment::None => MdTableAlignment::None,
                        comrak::nodes::TableAlignment::Left => MdTableAlignment::Left,
                        comrak::nodes::TableAlignment::Center => MdTableAlignment::Center,
                        comrak::nodes::TableAlignment::Right => MdTableAlignment::Right,
                    })
                    .collect(),
            },
            NodeValue::TableRow(row) => Markdown::TableRow { is_header: *row },
            NodeValue::TableCell => Markdown::TableCell,
            NodeValue::Text(text) => Markdown::Text {
                value: text.clone(),
            },
            NodeValue::TaskItem(item) => Markdown::TaskItem {
                checked: item.is_some(),
            },
            NodeValue::SoftBreak => Markdown::SoftBreak,
            NodeValue::LineBreak => Markdown::LineBreak,
            NodeValue::Code(code) => Markdown::CodeInline {
                value: code.literal.clone(),
            },
            NodeValue::HtmlInline(html) => Markdown::HtmlInline {
                value: html.clone(),
            },
            NodeValue::Emph => Markdown::Italic,
            NodeValue::Strong => Markdown::Bold,
            NodeValue::Strikethrough => Markdown::StrikeThrough,
            NodeValue::Superscript => Markdown::SuperScript,
            NodeValue::Link(link) => Markdown::Link {
                url: link.url.clone(),
                title: if link.title.is_empty() {
                    None
                } else {
                    Some(link.title.clone())
                },
            },
            NodeValue::Image(image) => Markdown::Image {
                url: image.url.clone(),
                title: if image.title.is_empty() {
                    None
                } else {
                    Some(image.title.clone())
                },
            },
            NodeValue::FootnoteReference(footnote_ref) => Markdown::FootnoteRef {
                id: footnote_ref.to_string(),
            },
            NodeValue::FootnoteDefinition(footnote) => Markdown::FootnoteDef {
                id: footnote.to_string(),
            },
        };

        // span
        let src_repos = &ast_root.data.borrow().sourcepos;
        let span = Span::new(
            src_repos.start.line,
            src_repos.start.column,
            src_repos.end.line,
            src_repos.end.column,
        );

        // children
        let mut children = vec![];
        for ast_child in ast_root.children() {
            let child = self.parse_node_iter(ast_child)?;
            children.push(child);
        }

        Ok(Node::new(data).with_span(span).with_children(children))
    }
}
