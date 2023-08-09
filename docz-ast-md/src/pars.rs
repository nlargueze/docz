//! Parsing

use comrak::{
    nodes::{AstNode, NodeValue},
    Arena, ComrakOptions,
};
use docz_ast::{AstParser, Error, Node, NodeType};
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

impl AstParser for MdParser {
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

        self.parse_node_iter(md_root, None)
    }
}

impl MdParser {
    /// Parses a node recursively
    #[allow(clippy::only_used_in_recursion)]
    fn parse_node_iter<'a>(
        &self,
        ast_root: &'a AstNode<'a>,
        parent: Option<&mut Node>,
    ) -> Result<Node, Error> {
        // eprintln!("ast_root: {:#?}", ast_root.data);

        let mut node = Node::default();
        match &ast_root.data.borrow().value {
            NodeValue::Document => {
                node.set_type(NodeType::Document {
                    title: None,
                    authors: vec![],
                });
            }
            NodeValue::FrontMatter(fmatter) => {
                // NB: FrontMatter is in YAML format
                if let Some(parent) = parent {
                    parent.add_attr("frontmatter", Some(fmatter));
                } else {
                    return Err(Error::new("Frontmatter must be a child of a Document"));
                }
            }
            NodeValue::BlockQuote => {
                node.set_type(NodeType::BlockQuote);
            }
            NodeValue::List(list) => match list.list_type {
                comrak::nodes::ListType::Bullet => {
                    node.set_type(NodeType::List {
                        ordered: false,
                        start: None,
                    });
                }
                comrak::nodes::ListType::Ordered => {
                    node.set_type(NodeType::List {
                        ordered: true,
                        start: Some(list.start),
                    });
                }
            },
            NodeValue::Item(_item) => {
                node.set_type(NodeType::ListItem);
            }
            NodeValue::DescriptionList => {
                node.set_type(NodeType::DescrList);
            }
            NodeValue::DescriptionItem(_item) => {
                node.set_type(NodeType::DescrItem);
            }
            NodeValue::DescriptionTerm => {
                node.set_type(NodeType::DescrTerm);
            }
            NodeValue::DescriptionDetails => {
                node.set_type(NodeType::DescrDetails);
            }
            NodeValue::CodeBlock(block) => {
                node.set_type(NodeType::CodeBlock {
                    lang: Some(block.info.clone()),
                })
                .set_value(block.literal.as_str());
            }
            NodeValue::HtmlBlock(block) => {
                node.set_type(NodeType::HtmlBlock).set_value(&block.literal);
            }
            NodeValue::Paragraph => {
                node.set_type(NodeType::Paragraph);
            }
            NodeValue::Heading(heading) => {
                node.set_type(NodeType::Heading {
                    level: heading.level,
                });
            }
            NodeValue::ThematicBreak => {
                node.set_type(NodeType::Paragraph);
            }
            NodeValue::Table(table) => {
                todo!("Table: {:#?}", table)
            }
            NodeValue::TableRow(row) => {
                todo!("TableRow: {:#?}", row)
            }
            NodeValue::TableCell => {
                todo!("TableCell: {:#?}", "TableCell")
            }
            NodeValue::Text(text) => {
                node.set_type(NodeType::Text).set_value(text);
            }
            NodeValue::TaskItem(item) => {
                todo!("TaskItem: {:#?}", item)
            }
            NodeValue::SoftBreak => {
                node.set_type(NodeType::SoftBreak);
            }
            NodeValue::LineBreak => {
                node.set_type(NodeType::LineBreak);
            }
            NodeValue::Code(code) => {
                node.set_type(NodeType::Code)
                    .set_value(code.literal.as_str());
            }
            NodeValue::HtmlInline(html) => {
                todo!("HtmlInline: {:#?}", html)
            }
            NodeValue::Emph => {
                node.set_type(NodeType::Italic);
            }
            NodeValue::Strong => {
                node.set_type(NodeType::Bold);
            }
            NodeValue::Strikethrough => {
                node.set_type(NodeType::StrikeThrough);
            }
            NodeValue::Superscript => {
                todo!("Superscript: {:#?}", "Superscript")
            }
            NodeValue::Link(link) => {
                node.set_type(NodeType::Link {
                    url: link.url.to_string(),
                    title: Some(link.title.to_string()),
                });
            }
            NodeValue::Image(image) => {
                node.set_type(NodeType::Image {
                    url: image.url.to_string(),
                    title: Some(image.title.to_string()),
                });
            }
            NodeValue::FootnoteReference(footnote_ref) => {
                node.set_type(NodeType::FootnoteRef).set_value(footnote_ref);
            }
            NodeValue::FootnoteDefinition(footnote) => {
                node.set_type(NodeType::Footnote)
                    .set_value(footnote.as_str());
            }
        };

        for ast_child in ast_root.children() {
            let child = self.parse_node_iter(ast_child, Some(&mut node))?;
            node.add_child(child);
        }

        Ok(node)
    }
}

/// Extracts the frontmatter inside the document node
pub fn extract_frontmatter<'a, T>(node: &'a Node) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    let fm = node
        .attrs
        .get("frontmatter")
        .ok_or_else(|| Error::new("No frontmatter found"))?;
    match fm {
        Some(fm) => serde_yaml::from_str::<T>(fm).map_err(|e| Error::new(&e.to_string())),
        None => Err(Error::new("No frontmatter found")),
    }
}
