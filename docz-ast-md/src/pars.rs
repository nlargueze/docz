//! Parsing

use comrak::{
    nodes::{AstNode, NodeValue},
    Arena, ComrakOptions,
};
use docz_ast::{Error, Node, NodeKind, Parser, Span};

use crate::FRONTMATTER_SEP;

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
        opts.extension.front_matter_delimiter = Some(FRONTMATTER_SEP.to_string());
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
        let mut node = Node::new(NodeKind::Fragment);

        // node span
        let src_repos = &ast_root.data.borrow().sourcepos;
        node.set_span(Span::new(
            src_repos.start.line,
            src_repos.start.column,
            src_repos.end.line,
            src_repos.end.column,
        ));

        // parse node
        match &ast_root.data.borrow().value {
            NodeValue::Document => {
                node.set_kind(NodeKind::Document);
            }
            NodeValue::FrontMatter(fmatter) => {
                let fmatter = fmatter.trim();
                let fmatter = fmatter
                    .strip_prefix(FRONTMATTER_SEP)
                    .ok_or(Error::new("invalid front matter prefix"))?;
                let fmatter = fmatter
                    .strip_suffix(FRONTMATTER_SEP)
                    .ok_or(Error::new("invalid front matter suffix"))?;
                let value = fmatter.trim();
                node.set_kind(NodeKind::FrontMatter).set_value(value);
            }
            NodeValue::BlockQuote => {
                node.set_kind(NodeKind::BlockQuote);
            }
            NodeValue::List(list) => match list.list_type {
                comrak::nodes::ListType::Bullet => {
                    node.set_kind(NodeKind::List { ordered: false });
                }
                comrak::nodes::ListType::Ordered => {
                    node.set_kind(NodeKind::List { ordered: true });
                }
            },
            NodeValue::Item(item) => {
                node.set_kind(NodeKind::ListItem {
                    index: if item.list_type == comrak::nodes::ListType::Ordered {
                        Some(item.start)
                    } else {
                        None
                    },
                });
            }
            NodeValue::DescriptionList => {
                node.set_kind(NodeKind::DefinitionList);
            }
            NodeValue::DescriptionItem(_item) => {
                node.set_kind(NodeKind::DefinitionItem);
            }
            NodeValue::DescriptionTerm => {
                node.set_kind(NodeKind::DefinitionTerm);
            }
            NodeValue::DescriptionDetails => {
                node.set_kind(NodeKind::DefinitionDetails);
            }
            NodeValue::CodeBlock(block) => {
                node.set_kind(NodeKind::CodeBlock {
                    info: block.info.clone(),
                })
                .set_value(&block.literal);
            }
            NodeValue::HtmlBlock(block) => {
                node.set_kind(NodeKind::Html).set_value(&block.literal);
            }
            NodeValue::Paragraph => {
                node.set_kind(NodeKind::Paragraph);
            }
            NodeValue::Heading(heading) => {
                node.set_kind(NodeKind::Heading {
                    level: heading.level,
                    id: None,
                });
            }
            NodeValue::ThematicBreak => {
                node.set_kind(NodeKind::ThematicBreak);
            }
            NodeValue::Table(_table) => {
                node.set_kind(NodeKind::Table);
            }
            NodeValue::TableRow(row) => {
                node.set_kind(NodeKind::TableRow { is_header: *row });
            }
            NodeValue::TableCell => {
                node.set_kind(NodeKind::TableCell);
            }
            NodeValue::Text(text) => {
                node.set_kind(NodeKind::Text).set_value(text);
            }
            NodeValue::TaskItem(item) => {
                node.set_kind(NodeKind::TaskItem {
                    checked: item.is_some(),
                });
            }
            NodeValue::SoftBreak => {
                node.set_kind(NodeKind::SoftBreak);
            }
            NodeValue::LineBreak => {
                node.set_kind(NodeKind::LineBreak);
            }
            NodeValue::Code(code) => {
                node.set_kind(NodeKind::Code).set_value(&code.literal);
            }
            NodeValue::HtmlInline(html) => {
                node.set_kind(NodeKind::Code).set_value(html);
            }
            NodeValue::Emph => {
                node.set_kind(NodeKind::Italic);
            }
            NodeValue::Strong => {
                node.set_kind(NodeKind::Bold);
            }
            NodeValue::Strikethrough => {
                node.set_kind(NodeKind::StrikeThrough);
            }
            NodeValue::Superscript => {
                node.set_kind(NodeKind::SuperScript);
            }
            NodeValue::Link(link) => {
                node.set_kind(NodeKind::Link {
                    url: link.url.clone(),
                    title: if link.title.is_empty() {
                        None
                    } else {
                        Some(link.title.clone())
                    },
                });
            }
            NodeValue::Image(image) => {
                node.set_kind(NodeKind::Image {
                    url: image.url.clone(),
                    title: if image.title.is_empty() {
                        None
                    } else {
                        Some(image.title.clone())
                    },
                });
            }
            NodeValue::FootnoteReference(footnote_ref) => {
                node.set_kind(NodeKind::FootnoteRef {
                    id: footnote_ref.to_string(),
                });
            }
            NodeValue::FootnoteDefinition(footnote) => {
                node.set_kind(NodeKind::FootnoteDef {
                    id: footnote.to_string(),
                });
            }
        };

        for ast_child in ast_root.children() {
            let child = self.parse_node_iter(ast_child)?;
            node.add_child(child);
        }

        Ok(node)
    }
}
