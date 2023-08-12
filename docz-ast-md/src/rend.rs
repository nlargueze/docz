//! Renderer

use std::fmt::Write;

use docz_ast::{Error, Node, NodeKind, Renderer};

use crate::FRONTMATTER_SEP;

/// AST renderer for markdown
#[derive(Debug, Default)]
pub struct MdRenderer {}

impl MdRenderer {
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }
}

impl Renderer for MdRenderer {
    fn is_binary(&self) -> bool {
        false
    }

    fn render(&self, node: &Node) -> Result<Vec<u8>, Error> {
        let mut buffer = RenderBuffer::default();
        self.render_node_iter(&mut buffer, node)?;
        Ok(buffer.data.into_bytes())
    }
}

/// Render buffer
#[derive(Debug, Default)]
struct RenderBuffer {
    /// Data
    data: String,
    /// List indent
    list_indent: Option<usize>,
    /// Ignore the paragraph indent
    ignore_paragraph_indent: bool,
}

impl RenderBuffer {
    /// Increments a list
    fn incr_list(&mut self) {
        if let Some(indent) = self.list_indent {
            self.list_indent = Some(indent + 1);
        } else {
            self.list_indent = Some(1);
        }
    }

    /// Decrements a list
    fn decr_list(&mut self) {
        if let Some(indent) = self.list_indent {
            if indent > 1 {
                self.list_indent = Some(indent - 1);
            } else {
                self.list_indent = None;
            }
        }
    }
}

impl Write for RenderBuffer {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.data.push_str(s);
        Ok(())
    }
}

impl MdRenderer {
    /// Renders a node iteratively
    fn render_node_iter(&self, buffer: &mut RenderBuffer, node: &Node) -> Result<(), Error> {
        let value = node.value.as_ref();
        let children = node.children.as_ref();
        match &node.kind {
            NodeKind::Document => {
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
            }
            NodeKind::Fragment => {
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
            }
            NodeKind::FrontMatter => {
                write!(buffer, "{}", FRONTMATTER_SEP)?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
                write!(buffer, "{}", FRONTMATTER_SEP)?;
                writeln!(buffer)?;
            }
            NodeKind::Chapter => {
                writeln!(buffer)?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
                writeln!(buffer)?;
            }
            NodeKind::Section => {
                writeln!(buffer)?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
                writeln!(buffer)?;
            }
            NodeKind::Heading { level, id } => {
                writeln!(buffer)?;
                write!(buffer, "{} ", "#".repeat((*level).into()))?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
                if let Some(id) = id {
                    write!(buffer, " {{#{}}}", id)?;
                }
                writeln!(buffer)?;
            }
            NodeKind::Paragraph => {
                if !buffer.ignore_paragraph_indent {
                    writeln!(buffer)?;
                }
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
                if !buffer.ignore_paragraph_indent {
                    writeln!(buffer)?;
                }
            }
            NodeKind::Text => {
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
            }
            NodeKind::Comment => {
                // NB: comments are not rendered
                // writeln!(buffer, "<!--")?;
                // self.render_value(buffer, value)?;
                // self.render_children(buffer, children)?;
                // writeln!(buffer, "-->")?;
            }
            NodeKind::ThematicBreak { .. } => {
                writeln!(buffer)?;
                writeln!(buffer, "---")?;
                writeln!(buffer)?;
            }
            NodeKind::LineBreak { .. } => {
                writeln!(buffer)?;
                writeln!(buffer)?;
            }
            NodeKind::SoftBreak => {
                writeln!(buffer)?;
            }
            NodeKind::Italic => {
                write!(buffer, "_")?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
                write!(buffer, "_")?;
            }
            NodeKind::Bold => {
                write!(buffer, "**")?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
                write!(buffer, "**")?;
            }
            NodeKind::BlockQuote => {
                // FIXME: add a > prefix
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
            }
            NodeKind::List { ordered: _ } => {
                buffer.ignore_paragraph_indent = true;
                buffer.incr_list();

                writeln!(buffer)?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;

                buffer.ignore_paragraph_indent = false;
                buffer.decr_list();
            }
            NodeKind::ListItem { index } => {
                let indent = if let Some(indent) = buffer.list_indent {
                    "  ".repeat(indent)
                } else {
                    "".to_string()
                };

                if let Some(idx) = index {
                    write!(buffer, "{indent}{idx}. ")?;
                } else {
                    write!(buffer, "{indent}- ")?;
                }
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
                writeln!(buffer)?;
            }
            NodeKind::Code => {
                write!(buffer, "`")?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
                write!(buffer, "`")?;
            }
            NodeKind::Link { url, title } => {
                // NB: the link text is contained inside the children
                let mut link_buf = RenderBuffer::default();
                self.render_children(&mut link_buf, children)?;
                write!(buffer, "[{}]", &link_buf.data)?;
                write!(buffer, "({url})")?;
                if let Some(title) = title {
                    write!(buffer, " \"{}\"", title)?;
                }
            }
            NodeKind::Image { url, title } => {
                // NB: the link text is contained inside the children
                let mut link_buf = RenderBuffer::default();
                self.render_children(&mut link_buf, children)?;
                write!(buffer, "![{}]", &link_buf.data)?;
                write!(buffer, "({url})")?;
                if let Some(title) = title {
                    write!(buffer, " \"{}\"", title)?;
                }
            }
            NodeKind::Html => {
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
            }
            NodeKind::Table => {
                writeln!(buffer)?;
                writeln!(buffer)?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
            }
            NodeKind::TableRow { is_header: _ } => {
                write!(buffer, "| ")?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
                write!(buffer, "| ----- | ")?;
            }
            NodeKind::TableCell => {
                write!(buffer, "| ")?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
            }
            NodeKind::CodeBlock { info } => {
                writeln!(buffer)?;
                writeln!(buffer, "```{info}")?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
                writeln!(buffer, "```")?;
                writeln!(buffer)?;
            }
            NodeKind::FootnoteRef { id } => {
                write!(buffer, "[^{}] ", id)?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
            }
            NodeKind::FootnoteDef { id } => {
                writeln!(buffer)?;
                write!(buffer, "[^{}]: ", id)?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
            }
            NodeKind::DefinitionList => {
                writeln!(buffer)?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
            }
            NodeKind::DefinitionItem => {
                writeln!(buffer)?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
            }
            NodeKind::DefinitionTerm => {
                writeln!(buffer)?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
            }
            NodeKind::DefinitionDetails => {
                write!(buffer, ": ")?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
            }
            NodeKind::StrikeThrough => {
                write!(buffer, "~~")?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
                write!(buffer, "~~")?;
            }
            NodeKind::TaskItem { checked } => {
                let check = match *checked {
                    false => " ",
                    true => "x",
                };
                write!(buffer, "- [{check}] ")?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
                writeln!(buffer)?;
            }
            NodeKind::Highlight => {
                write!(buffer, "==")?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
                write!(buffer, "==")?;
            }
            NodeKind::SubScript => {
                write!(buffer, "~")?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
                write!(buffer, "~")?;
            }
            NodeKind::SuperScript => {
                write!(buffer, "^")?;
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
                write!(buffer, "^")?;
            }
            NodeKind::Other { name: _, .. } => {
                // Other nodes are excluded
                self.render_value(buffer, value)?;
                self.render_children(buffer, children)?;
            }
        }

        Ok(())
    }

    /// Renders the value
    fn render_value(&self, buffer: &mut RenderBuffer, value: Option<&String>) -> Result<(), Error> {
        if let Some(value) = value {
            write!(buffer, "{}", value)?;
        }
        Ok(())
    }

    /// Renders the children
    fn render_children(
        &self,
        buffer: &mut RenderBuffer,
        children: Option<&Vec<Node>>,
    ) -> Result<(), Error> {
        if let Some(children) = children {
            for child in children {
                self.render_node_iter(buffer, child)?;
            }
        }
        Ok(())
    }
}
