//! Renderer

use docz_ast::{Error, Node, Renderer};

use crate::Html;

/// AST renderer for markdown
#[derive(Debug, Default)]
pub struct HTMLRenderer {
    /// Document title
    pub doc_title: Option<String>,
    /// Document authors
    pub doc_authors: Option<Vec<String>>,
}

impl HTMLRenderer {
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }
}

impl Renderer<Html> for HTMLRenderer {
    fn render(&self, node: &Node<Html>) -> Result<Vec<u8>, Error> {
        Ok(self.render_node_iter(node)?.into_bytes())
    }
}

impl HTMLRenderer {
    // Renders a node recursively
    #[allow(clippy::only_used_in_recursion)]
    fn render_node_iter(&self, node: &Node<Html>) -> Result<String, Error> {
        let mut children_html = vec![];
        for child in &node.children {
            let child_html = self.render_node_iter(child)?;
            children_html.push(child_html);
        }
        let mut children = children_html.join("");

        let html = match &node.data {
            Html::Fragment => children,
            Html::Text { value } => value.to_string(),
            Html::Comment { value } => format!("<!--{value}-->"),
            Html::Element {
                tag,
                void,
                id,
                attrs,
                classes,
            } => {
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

                format!("{open_tag}{children}{close_tag}")
            }
        };

        // sanitize ?

        Ok(html)
    }
}
