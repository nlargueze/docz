//! Markdown AST for docz.

use comrak::{
    nodes::{AstNode, NodeValue},
    parse_document, Arena, ComrakOptions,
};
use docz_ast::{Node, NodeType, Parser, Renderer};
use error::Error;

pub mod error;
#[cfg(test)]
mod tests;

/// AST parser for markdown
#[derive(Debug, Default)]
pub struct MdConverter {
    /// Front matter
    front_matter: Option<String>,
}

impl MdConverter {
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }
}

impl Parser for MdConverter {
    type Err = Error;

    fn parse(&self, data: &str) -> Result<Node, Self::Err> {
        let arena = Arena::new();
        let mut opts = ComrakOptions::default();
        opts.extension.strikethrough = true;
        opts.extension.tagfilter = true;
        opts.extension.table = true;
        opts.extension.autolink = true;
        opts.extension.tasklist = true;
        opts.extension.footnotes = true;
        opts.extension.front_matter_delimiter = Some("---".to_string());
        let md_root = parse_document(&arena, data, &opts);
        // eprintln!("{md_root:#?}");

        let node = self.parse_node_iter(md_root)?;
        Ok(node)
    }
}

impl MdConverter {
    /// Parses a node in a recursive manner
    #[allow(clippy::only_used_in_recursion)]
    fn parse_node_iter<'a>(&self, ast_root: &'a AstNode<'a>) -> Result<Node, Error> {
        let mut node = Node::default();
        eprintln!("ast_root: {:#?}", ast_root.data);

        match &ast_root.data.borrow().value {
            NodeValue::Document => {}
            NodeValue::FrontMatter(fmatter) => {
                // NB: FrontMatter is in YAML format
                eprintln!("Frontmatter: {:#?}", fmatter);
                node.ty(NodeType::FrontMatter);
                node.content(fmatter);
            }
            NodeValue::BlockQuote => {
                node.ty(NodeType::FrontMatter);
                node.tag("blockquote");
            }
            NodeValue::List(list) => match list.list_type {
                comrak::nodes::ListType::Bullet => {
                    node.ty(NodeType::UnorderedList);
                    node.tag("ul");
                }
                comrak::nodes::ListType::Ordered => {
                    node.ty(NodeType::OrderedList { start: list.start });
                    node.tag("ol");
                }
            },
            NodeValue::Item(_item) => {
                node.ty(NodeType::ListItem);
                node.tag("li");
            }
            NodeValue::DescriptionList => {
                todo!("DescriptionList: {:#?}", "DescriptionList")
            }
            NodeValue::DescriptionItem(item) => {
                todo!("DescriptionItem: {:#?}", item);
            }
            NodeValue::DescriptionTerm => {
                todo!("DescriptionTerm: {:#?}", "DescriptionTerm")
            }
            NodeValue::DescriptionDetails => {
                todo!("DescriptionDetails: {:#?}", "DescriptionDetails")
            }
            NodeValue::CodeBlock(block) => {
                node.ty(NodeType::CodeBlock {
                    lang: block.info.clone(),
                });
                node.tag("code");
                node.content(block.literal.as_str());
            }
            NodeValue::HtmlBlock(block) => {
                todo!("HtmlBlock: {:#?}", block)
            }
            NodeValue::Paragraph => {
                node.ty(NodeType::Paragraph);
                node.tag("p");
            }
            NodeValue::Heading(heading) => {
                node.ty(NodeType::Heading(heading.level));
                node.tag(format!("h{}", heading.level).as_str());
                // root = root.child(child)

                //  {
                //     ty: NodeType::Heading {
                //         level: heading.level,
                //     },
                //     tag: todo!(),
                //     attributes: todo!(),
                //     content: todo!(),
                //     children: todo!(),
                // };
            }
            NodeValue::ThematicBreak => {
                node.ty(NodeType::Paragraph);
                node.tag("hr");
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
                node.ty(NodeType::Text);
                node.content(text);
            }
            NodeValue::TaskItem(item) => {
                todo!("TaskItem: {:#?}", item)
            }
            NodeValue::SoftBreak => {
                // soft break
                node.ty(NodeType::SoftBreak);
            }
            NodeValue::LineBreak => {
                // line break
                node.ty(NodeType::LineBreak);
            }
            NodeValue::Code(code) => {
                // code
                node.ty(NodeType::Code);
                node.tag("code");
                node.content(code.literal.as_str());
            }
            NodeValue::HtmlInline(html) => {
                todo!("HtmlInline: {:#?}", html)
            }
            NodeValue::Emph => {
                // emphasis
                node.ty(NodeType::Italic);
                node.tag("i");
            }
            NodeValue::Strong => {
                // bold
                node.ty(NodeType::Bold);
                node.tag("b");
            }
            NodeValue::Strikethrough => {
                // strikethrough
                node.ty(NodeType::StrikeThrough);
                node.tag("s");
            }
            NodeValue::Superscript => {
                todo!("Superscript: {:#?}", "Superscript")
            }
            NodeValue::Link(link) => {
                node.ty(NodeType::Link {
                    url: link.url.to_string(),
                    title: link.title.to_string(),
                });
                node.tag("a");
            }
            NodeValue::Image(image) => {
                node.ty(NodeType::Link {
                    url: image.url.to_string(),
                    title: image.title.to_string(),
                });
                node.tag("img");
            }
            NodeValue::FootnoteReference(footnote_ref) => {
                node.ty(NodeType::FootnoteRef(footnote_ref.clone()));
            }
            NodeValue::FootnoteDefinition(footnote) => {
                node.ty(NodeType::Footnote);
                node.content(footnote.as_str());
            }
        }

        for ast_child in ast_root.children() {
            let child = self.parse_node_iter(ast_child)?;
            node.child(child)
        }

        Ok(node)
    }
}

impl Renderer for MdConverter {
    type Err = Error;

    fn render(&self, node: &Node) -> Result<String, Self::Err> {
        let mut data = String::new();

        todo!("render");

        Ok(data)
    }
}
