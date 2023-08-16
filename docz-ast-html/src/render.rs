//! Renderer

use crate::{Error, HtmlNode};

/// HTML render Options
#[derive(Debug, Clone, Default)]
pub struct RenderOptions {}

impl HtmlNode {
    pub fn render(&self, opts: RenderOptions) -> Result<String, Error> {
        render_node_iter(self, &opts)
    }
}

/// Renders a node iteratively
fn render_node_iter(node: &HtmlNode, opts: &RenderOptions) -> Result<String, Error> {
    match node {
        HtmlNode::Document { children } => {
            let children = render_node_children_iter(children, opts)?;
            Ok(format!("<!DOCTYPE html>\n{children}"))
        }
        HtmlNode::Fragment { children } => render_node_children_iter(children, opts),
        HtmlNode::Text { value } => Ok(value.to_string()),
        HtmlNode::Comment { value } => Ok(format!("<!--{value}-->")),
        HtmlNode::Element {
            tag,
            void,
            id,
            attrs,
            classes,
            children,
        } => {
            let mut children = render_node_children_iter(children, opts)?;

            let id = id
                .as_ref()
                .map(|id| format!(" id=\"{}\"", id))
                .unwrap_or_default();

            let mut attrs_str = String::new();
            for (key, value) in attrs {
                attrs_str.push_str(
                    format!(
                        " {key}{}",
                        if let Some(v) = value {
                            format!("=\"{v}\"")
                        } else {
                            "".to_string()
                        }
                    )
                    .as_str(),
                );
            }

            let mut class = String::new();
            if !classes.is_empty() {
                class = format!("class=\"{}\"", classes.join(" "));
            }

            let mut open_tag = format!("<{tag}{id}{attrs_str}{class}>");
            let mut close_tag = format!("</{tag}>");

            if *void {
                open_tag = open_tag.strip_suffix('>').unwrap().to_string();
                open_tag.push_str(" />");
                children = "".to_string();
                close_tag = "".to_string();
            }

            Ok(format!("{open_tag}{children}{close_tag}"))
        }
    }
}

/// Renders a node children recursively
fn render_node_children_iter(children: &[HtmlNode], opts: &RenderOptions) -> Result<String, Error> {
    let mut children_str = vec![];
    for child in children.iter() {
        let child_str = render_node_iter(child, opts)?;
        children_str.push(child_str);
    }
    Ok(children_str.join(""))
}
