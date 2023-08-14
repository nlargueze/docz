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

use crate::{Error, MdNode, MdTableAlignment, FRONTMATTER_SEP};

/// Render options
#[derive(Debug, Clone, Default)]
pub struct RenderOptions {}

impl MdNode {
    /// Renders to MdNode
    pub fn render(&self, opts: RenderOptions) -> Result<String, Error> {
        let arena = Arena::new();
        let md_node = render_node_iter(self, &arena, &opts);

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
            format_commonmark(md_node, &comrak_opts, &mut buffer)?;
            let buffer_str = String::from_utf8(buffer)?;
            Ok(buffer_str)
        } else {
            Ok("".to_string())
        }
    }
}

/// Converts an AST node to a MdNode node
fn render_node_iter<'a>(
    node: &MdNode,
    arena: &'a Arena<AstNode<'a>>,
    opts: &RenderOptions,
) -> Option<&'a AstNode<'a>> {
    let mut children_ref: Option<&Vec<MdNode>> = None;

    let md_node_value = match &node {
        MdNode::Document { children } => {
            children_ref = Some(children);
            NodeValue::Document
        }
        MdNode::FrontMatter { value } => {
            let fmatter = format!("{}\n{}\n{}\n\n", FRONTMATTER_SEP, value, FRONTMATTER_SEP);
            NodeValue::FrontMatter(fmatter)
        }
        MdNode::Heading { level, id: _ } => NodeValue::Heading(NodeHeading {
            level: *level,
            setext: false,
        }),
        MdNode::Paragraph { children } => {
            children_ref = Some(children);
            NodeValue::Paragraph
        }
        MdNode::Text { value } => NodeValue::Text(value.to_string()),
        MdNode::Comment { value: _ } => NodeValue::Text("".to_string()),
        MdNode::ThematicBreak => NodeValue::ThematicBreak,
        MdNode::LineBreak => NodeValue::LineBreak,
        MdNode::SoftBreak => NodeValue::SoftBreak,
        MdNode::Italic { children } => {
            children_ref = Some(children);
            NodeValue::Emph
        }
        MdNode::Bold { children } => {
            children_ref = Some(children);
            NodeValue::Strong
        }
        MdNode::BlockQuote { children } => {
            children_ref = Some(children);
            NodeValue::BlockQuote
        }
        MdNode::List { ordered, children } => {
            children_ref = Some(children);
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
        MdNode::ListItem { index, children } => {
            children_ref = Some(children);
            NodeValue::Item(NodeList {
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
            })
        }
        MdNode::CodeInline { value } => NodeValue::Code(NodeCode {
            literal: value.to_string(),
            num_backticks: 1,
        }),
        MdNode::CodeBlock { info, value } => NodeValue::CodeBlock(NodeCodeBlock {
            literal: value.to_string(),
            fenced: true,
            fence_char: '`'.try_into().expect("invalid fence char"),
            fence_length: 3,
            fence_offset: 0,
            info: info.clone(),
        }),
        MdNode::Link { url, title } => NodeValue::Link(NodeLink {
            url: url.clone(),
            title: title.clone().unwrap_or_default(),
        }),
        MdNode::Image { url, title } => NodeValue::Image(NodeLink {
            url: url.clone(),
            title: title.clone().unwrap_or_default(),
        }),
        MdNode::HtmlInline { value } => NodeValue::HtmlInline(value.to_string()),
        MdNode::HtmlBlock { value } => NodeValue::HtmlBlock(NodeHtmlBlock {
            literal: value.to_string(),
            block_type: ' '.try_into().expect("invalid fence char"),
        }),
        MdNode::Table { columns, children } => {
            children_ref = Some(children);
            NodeValue::Table(
                columns
                    .iter()
                    .map(|align| match align {
                        MdTableAlignment::None => comrak::nodes::TableAlignment::None,
                        MdTableAlignment::Left => comrak::nodes::TableAlignment::Left,
                        MdTableAlignment::Center => comrak::nodes::TableAlignment::Center,
                        MdTableAlignment::Right => comrak::nodes::TableAlignment::Right,
                    })
                    .collect(),
            )
        }
        MdNode::TableRow {
            is_header,
            children,
        } => {
            children_ref = Some(children);
            NodeValue::TableRow(*is_header)
        }
        MdNode::TableCell { children } => {
            children_ref = Some(children);
            NodeValue::TableCell
        }
        MdNode::FootnoteRef { id } => NodeValue::FootnoteReference(id.clone()),
        MdNode::FootnoteDef { id } => NodeValue::FootnoteDefinition(id.clone()),
        MdNode::DefinitionList { children } => {
            children_ref = Some(children);
            NodeValue::DescriptionList
        }
        MdNode::DefinitionItem { children } => {
            children_ref = Some(children);
            NodeValue::DescriptionItem(NodeDescriptionItem {
                marker_offset: 0,
                padding: 0,
            })
        }
        MdNode::DefinitionTerm { children } => {
            children_ref = Some(children);
            NodeValue::DescriptionTerm
        }
        MdNode::DefinitionDetails { children } => {
            children_ref = Some(children);
            NodeValue::DescriptionDetails
        }
        MdNode::StrikeThrough { children } => {
            children_ref = Some(children);
            NodeValue::Strikethrough
        }
        MdNode::TaskItem { checked, children } => {
            children_ref = Some(children);
            NodeValue::TaskItem(if *checked { Some('x') } else { Some(' ') })
        }
        MdNode::Highlight { children: _ } => {
            // children_ref = Some(children);
            return None;
        }
        MdNode::SubScript { children: _ } => {
            // children_ref = Some(children);
            return None;
        }
        MdNode::SuperScript { children: _ } => {
            // children_ref = Some(children);
            NodeValue::Superscript
        }
    };

    let line_col: LineColumn = (0, 0).into();
    let md_ast = Ast::new(md_node_value, line_col);
    let md_cell = RefCell::new(md_ast);
    let md_node = AstNode::new(md_cell);
    let md_node = arena.alloc(md_node);

    if let Some(children) = children_ref {
        for child in children {
            if let Some(md_child) = render_node_iter(child, arena, opts) {
                md_node.append(md_child);
            }
        }
    }

    Some(md_node)
}

/// Renders the frontmatter YAML string
pub fn render_frontmatter<T>(value: &T) -> Result<String, Error>
where
    T: serde::Serialize,
{
    serde_yaml::to_string(value).map_err(|e| Error::new(&e.to_string()))
}
