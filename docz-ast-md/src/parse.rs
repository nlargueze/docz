//! Parsing

use comrak::{
    nodes::{AstNode, NodeValue},
    Arena, ComrakExtensionOptions, ComrakOptions, ComrakParseOptions, ComrakRenderOptions,
};
use serde::Deserialize;

use crate::{Error, MdNode, MdTableAlignment, FRONTMATTER_SEP};

/// Parse options
#[derive(Debug, Clone, Default)]
pub struct ParseOptions {}

impl MdNode {
    /// Parses a node
    pub fn parse(buffer: &str, opts: ParseOptions) -> Result<Self, Error> {
        let arena = Arena::new();
        let comrak_opts = ComrakOptions {
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
        let comrak_node = comrak::parse_document(&arena, buffer, &comrak_opts);
        parse_node_iter(comrak_node, &opts)
    }
}

/// Parses a comrak node recursively
#[allow(clippy::only_used_in_recursion)]
fn parse_node_iter<'a>(comrak_node: &'a AstNode<'a>, opts: &ParseOptions) -> Result<MdNode, Error> {
    // // span
    // let src_repos = &ast_root.data.borrow().sourcepos;
    // let span = Span::new(
    //     src_repos.start.line,
    //     src_repos.start.column,
    //     src_repos.end.line,
    //     src_repos.end.column,
    // );

    // children
    let mut children = vec![];
    for ast_child in comrak_node.children() {
        let child = parse_node_iter(ast_child, opts)?;
        children.push(child);
    }

    let node = match &comrak_node.data.borrow().value {
        NodeValue::Document => MdNode::Document { children },
        NodeValue::FrontMatter(fmatter) => {
            let fmatter = fmatter.trim();
            let fmatter = fmatter
                .strip_prefix(FRONTMATTER_SEP)
                .ok_or(Error::new("invalid front matter prefix"))?;
            let fmatter = fmatter
                .strip_suffix(FRONTMATTER_SEP)
                .ok_or(Error::new("invalid front matter suffix"))?;
            let value = fmatter.trim();
            MdNode::FrontMatter {
                value: value.to_string(),
            }
        }
        NodeValue::BlockQuote => MdNode::BlockQuote { children },
        NodeValue::List(list) => match list.list_type {
            comrak::nodes::ListType::Bullet => MdNode::List {
                ordered: false,
                children,
            },
            comrak::nodes::ListType::Ordered => MdNode::List {
                ordered: true,
                children,
            },
        },
        NodeValue::Item(item) => MdNode::ListItem {
            index: if item.list_type == comrak::nodes::ListType::Ordered {
                Some(item.start)
            } else {
                None
            },
            children,
        },
        NodeValue::DescriptionList => MdNode::DefinitionList { children },
        NodeValue::DescriptionItem(_item) => MdNode::DefinitionItem { children },
        NodeValue::DescriptionTerm => MdNode::DefinitionTerm { children },
        NodeValue::DescriptionDetails => MdNode::DefinitionDetails { children },
        NodeValue::CodeBlock(block) => MdNode::CodeBlock {
            info: block.info.clone(),
            value: block.literal.clone(),
        },
        NodeValue::HtmlBlock(block) => MdNode::HtmlBlock {
            value: block.literal.clone(),
        },
        NodeValue::Paragraph => MdNode::Paragraph { children },
        NodeValue::Heading(heading) => MdNode::Heading {
            level: heading.level,
            id: None,
        },
        NodeValue::ThematicBreak => MdNode::ThematicBreak,
        NodeValue::Table(table) => MdNode::Table {
            columns: table
                .iter()
                .map(|align| match align {
                    comrak::nodes::TableAlignment::None => MdTableAlignment::None,
                    comrak::nodes::TableAlignment::Left => MdTableAlignment::Left,
                    comrak::nodes::TableAlignment::Center => MdTableAlignment::Center,
                    comrak::nodes::TableAlignment::Right => MdTableAlignment::Right,
                })
                .collect(),
            children,
        },
        NodeValue::TableRow(row) => MdNode::TableRow {
            is_header: *row,
            children,
        },
        NodeValue::TableCell => MdNode::TableCell { children },
        NodeValue::Text(text) => MdNode::Text {
            value: text.clone(),
        },
        NodeValue::TaskItem(item) => MdNode::TaskItem {
            checked: item.is_some(),
            children,
        },
        NodeValue::SoftBreak => MdNode::SoftBreak,
        NodeValue::LineBreak => MdNode::LineBreak,
        NodeValue::Code(code) => MdNode::CodeInline {
            value: code.literal.clone(),
        },
        NodeValue::HtmlInline(html) => MdNode::HtmlInline {
            value: html.clone(),
        },
        NodeValue::Emph => MdNode::Italic { children },
        NodeValue::Strong => MdNode::Bold { children },
        NodeValue::Strikethrough => MdNode::StrikeThrough { children },
        NodeValue::Superscript => MdNode::SuperScript { children },
        NodeValue::Link(link) => MdNode::Link {
            url: link.url.clone(),
            title: if link.title.is_empty() {
                None
            } else {
                Some(link.title.clone())
            },
        },
        NodeValue::Image(image) => MdNode::Image {
            url: image.url.clone(),
            title: if image.title.is_empty() {
                None
            } else {
                Some(image.title.clone())
            },
        },
        NodeValue::FootnoteReference(footnote_ref) => MdNode::FootnoteRef {
            id: footnote_ref.to_string(),
        },
        NodeValue::FootnoteDefinition(footnote) => MdNode::FootnoteDef {
            id: footnote.to_string(),
        },
    };

    Ok(node)
}

/// Parses the frontmatter YAML string
pub fn parse_frontmatter<'a, T>(value: &'a str) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    serde_yaml::from_str::<T>(value).map_err(|e| Error::new(&e.to_string()))
}
