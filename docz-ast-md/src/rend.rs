//! Renderer

use core::cell::RefCell;

use comrak::{
    format_commonmark,
    nodes::{
        Ast, AstNode, LineColumn, ListDelimType, ListType, NodeCode, NodeCodeBlock,
        NodeDescriptionItem, NodeHeading, NodeHtmlBlock, NodeLink, NodeList, NodeValue,
    },
    Arena, ComrakExtensionOptions, ComrakOptions, ComrakParseOptions, ComrakRenderOptions,
    ListStyleType,
};
use docz_ast::{Error, Node, Renderer};

use crate::{Markdown, MdTableAlignment, FRONTMATTER_SEP};

/// AST renderer for markdown
#[derive(Debug, Default)]
pub struct MdRenderer {}

impl MdRenderer {
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }
}

impl Renderer<Markdown> for MdRenderer {
    fn render(&self, node: &Node<Markdown>) -> Result<Vec<u8>, Error> {
        let arena = Arena::new();
        let md_node = self.render_node_iter(node, &arena);

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
            render: ComrakRenderOptions {
                hardbreaks: false,
                github_pre_lang: true,
                full_info_string: true,
                width: 0,
                unsafe_: true,
                escape: true,
                list_style: ListStyleType::Dash,
                sourcepos: true,
            },
        };

        if let Some(md_node) = &md_node {
            let mut buffer = vec![];
            format_commonmark(md_node, &opts, &mut buffer)?;
            Ok(buffer)
        } else {
            Ok(vec![])
        }
    }
}

impl MdRenderer {
    /// Converts an AST node to a markdown node
    #[allow(clippy::only_used_in_recursion)]
    fn render_node_iter<'a>(
        &self,
        node: &Node<Markdown>,
        arena: &'a Arena<AstNode<'a>>,
    ) -> Option<&'a AstNode<'a>> {
        // node value
        let md_node_value = match &node.data {
            Markdown::Document => NodeValue::Document,
            Markdown::Fragment => NodeValue::Document,
            Markdown::FrontMatter { value } => {
                let fmatter = format!("{}\n{}\n{}\n\n", FRONTMATTER_SEP, value, FRONTMATTER_SEP);
                NodeValue::FrontMatter(fmatter)
            }
            Markdown::Chapter => NodeValue::Paragraph,
            Markdown::Section => NodeValue::Paragraph,
            Markdown::Heading { level, id: _ } => NodeValue::Heading(NodeHeading {
                level: *level,
                setext: false,
            }),
            Markdown::Paragraph => NodeValue::Paragraph,
            Markdown::Text { value } => NodeValue::Text(value.to_string()),
            Markdown::Comment { value: _ } => NodeValue::Text("".to_string()),
            Markdown::ThematicBreak => NodeValue::ThematicBreak,
            Markdown::LineBreak => NodeValue::LineBreak,
            Markdown::SoftBreak => NodeValue::SoftBreak,
            Markdown::Italic => NodeValue::Emph,
            Markdown::Bold => NodeValue::Strong,
            Markdown::BlockQuote => NodeValue::BlockQuote,
            Markdown::List { ordered } => {
                if *ordered {
                    NodeValue::List(NodeList {
                        list_type: ListType::Ordered,
                        marker_offset: 0,
                        padding: 0,
                        start: 1,
                        delimiter: ListDelimType::Period,
                        bullet_char: 42,
                        tight: true,
                    })
                } else {
                    NodeValue::List(NodeList {
                        list_type: ListType::Bullet,
                        marker_offset: 0,
                        padding: 0,
                        start: 1,
                        delimiter: ListDelimType::Period,
                        bullet_char: 42,
                        tight: true,
                    })
                }
            }
            Markdown::ListItem { index } => NodeValue::Item(NodeList {
                list_type: if index.is_some() {
                    ListType::Ordered
                } else {
                    ListType::Bullet
                },
                marker_offset: 0,
                padding: 0,
                start: if let Some(index) = index { *index } else { 1 },
                delimiter: ListDelimType::Period,
                bullet_char: 42,
                tight: true,
            }),
            Markdown::CodeInline { value } => NodeValue::Code(NodeCode {
                literal: value.to_string(),
                num_backticks: 1,
            }),
            Markdown::CodeBlock { info, value } => NodeValue::CodeBlock(NodeCodeBlock {
                literal: value.to_string(),
                fenced: true,
                fence_char: '`'.try_into().expect("invalid fence char"),
                fence_length: 3,
                fence_offset: 0,
                info: info.clone(),
            }),
            Markdown::Link { url, title } => NodeValue::Link(NodeLink {
                url: url.clone(),
                title: title.clone().unwrap_or_default(),
            }),
            Markdown::Image { url, title } => NodeValue::Image(NodeLink {
                url: url.clone(),
                title: title.clone().unwrap_or_default(),
            }),
            Markdown::HtmlInline { value } => NodeValue::HtmlInline(value.to_string()),
            Markdown::HtmlBlock { value } => NodeValue::HtmlBlock(NodeHtmlBlock {
                literal: value.to_string(),
                block_type: ' '.try_into().expect("invalid fence char"),
            }),
            Markdown::Table { columns } => NodeValue::Table(
                columns
                    .iter()
                    .map(|align| match align {
                        MdTableAlignment::None => comrak::nodes::TableAlignment::None,
                        MdTableAlignment::Left => comrak::nodes::TableAlignment::Left,
                        MdTableAlignment::Center => comrak::nodes::TableAlignment::Center,
                        MdTableAlignment::Right => comrak::nodes::TableAlignment::Right,
                    })
                    .collect(),
            ),
            Markdown::TableRow { is_header } => NodeValue::TableRow(*is_header),
            Markdown::TableCell => NodeValue::TableCell,
            Markdown::FootnoteRef { id } => NodeValue::FootnoteReference(id.clone()),
            Markdown::FootnoteDef { id } => NodeValue::FootnoteDefinition(id.clone()),
            Markdown::DefinitionList => NodeValue::DescriptionList,
            Markdown::DefinitionItem => NodeValue::DescriptionItem(NodeDescriptionItem {
                marker_offset: 0,
                padding: 0,
            }),
            Markdown::DefinitionTerm => NodeValue::DescriptionTerm,
            Markdown::DefinitionDetails => NodeValue::DescriptionDetails,
            Markdown::StrikeThrough => NodeValue::Strikethrough,
            Markdown::TaskItem { checked } => {
                NodeValue::TaskItem(if *checked { Some('x') } else { Some(' ') })
            }
            Markdown::Highlight => return None,
            Markdown::SubScript => return None,
            Markdown::SuperScript => NodeValue::Superscript,
            Markdown::Other { name: _ } => return None,
        };

        let line_col: LineColumn = if let Some(span) = &node.span {
            (span.start_line, span.start_col).into()
        } else {
            (0, 0).into()
        };

        let md_ast = Ast::new(md_node_value, line_col);
        let md_cell = RefCell::new(md_ast);
        let md_node = AstNode::new(md_cell);
        let md_node = arena.alloc(md_node);

        for child in &node.children {
            if let Some(md_child) = self.render_node_iter(child, arena) {
                md_node.append(md_child);
            }
        }

        Some(md_node)
    }
}
