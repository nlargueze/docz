//! Renderer

use docz_ast::{Error, Node, Renderer};
use maud::{html, DOCTYPE};

/// AST renderer for markdown
#[derive(Debug, Default)]
pub struct HTMLRenderer {}

impl HTMLRenderer {
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }
}

impl Renderer for HTMLRenderer {
    fn render(&self, node: &Node) -> Result<String, Error> {
        self.render_node_iter(node)
    }
}

impl HTMLRenderer {
    // Renders a node recursively
    #[allow(clippy::only_used_in_recursion)]
    fn render_node_iter(&self, node: &Node) -> Result<String, Error> {
        let html = match node {
            Node::Document {
                position: _,
                children,
                attrs: _,
                title,
                summary: _,
                authors: _,
            } => {
                let title = title.clone().unwrap_or("Title".to_string());
                let children = self.render_children(children)?;
                html! {
                    (DOCTYPE)
                    html {
                        head {
                            title { (title) }
                        }
                        body {
                            (children)
                        }
                    }
                }
            }
            // NodeType::Page => Ok(formatdoc! {"
            // //     <div x-tag=\"page\">
            // //     {children}
            // //     </div>
            // // "}),
            _ => {
                html! {}
            }
            // Node::Section => Ok(formatdoc! {"
            //     <section>
            //     {children}
            //     </section>
            // "}),
            // NodeType::Heading { level } => Ok(formatdoc! {"
            //     <h{level}>
            //     {children}
            //     </h{level}>
            // "}),
            // NodeType::Paragraph => Ok(formatdoc! {"
            //     <p>
            //     {children}
            //     </p>
            // "}),
            // NodeType::Row => Ok(formatdoc! {"
            //     <div>
            //     {children}
            //     </div>
            // "}),
            // NodeType::PageBreak => Ok(formatdoc! {"
            //     <br>
            // "}),
            // NodeType::LineBreak => Ok(formatdoc! {"
            //     <br>
            // "}),
            // NodeType::SoftBreak => Ok(formatdoc! {"
            //     &shy;
            // "}),
            // NodeType::Divider => Ok(formatdoc! {"
            //     <hr/>
            // "}),
            // NodeType::List { ordered, start: _ } => {
            //     if *ordered {
            //         Ok(formatdoc! {"
            //             <ul>
            //             {children}
            //             </ul>
            //         "})
            //     } else {
            //         Ok(formatdoc! {"
            //             <ol>
            //             {children}
            //             </ol>
            //         "})
            //     }
            // }
            // NodeType::ListItem => Ok(formatdoc! {"
            //     <li>
            //     {children}
            //     </li>
            // "}),
            // NodeType::Table => todo!(),
            // NodeType::TableRow => todo!(),
            // NodeType::FootnoteRef => todo!(),
            // NodeType::Footnote => todo!(),
            // NodeType::DescrList => Ok(formatdoc! {"
            //     <dl>
            //     {children}
            //     </dl>
            // "}),
            // NodeType::DescrItem => Ok(formatdoc! {"
            //     <dt>
            //     {children}
            //     </dt>
            // "}),
            // NodeType::DescrTerm => Ok(formatdoc! {"
            //     <dt>
            //     {children}
            //     </dt>
            // "}),
            // NodeType::DescrDetails => Ok(formatdoc! {"
            //     <dd>
            //     {children}
            //     </dd>
            // "}),
            // NodeType::Link { url, title: _ } => Ok(formatdoc! {"
            //     <a href=\"{url}\">
            //     {children}
            //     </a>
            // "}),
            // NodeType::Image { url, title } => {
            //     let title = title.clone().unwrap_or("".to_string());
            //     Ok(formatdoc! {"
            //         <img src=\"{url}\" alt=\"{title}\">
            //     "})
            // }
            // NodeType::CodeBlock { lang } => {
            //     let lang = lang.clone().unwrap_or("".to_string());
            //     Ok(formatdoc! {"
            //         <pre>
            //         <code class=\"language-{lang}\">
            //         {children}
            //         </code>
            //         </pre>
            //     "})
            // }
            // NodeType::BlockQuote => Ok(formatdoc! {"
            //     <blockquote>
            //     {children}
            //     </blockquote>
            // "}),
            // NodeType::HtmlBlock => Ok(children),
            // NodeType::Text => Ok(value),
            // NodeType::Comment => Ok(formatdoc! {"
            //     <!-- {value} -->
            // "}),
            // NodeType::Italic => Ok(formatdoc! {"
            //     <i>
            //     {children}
            //     </i>
            // "}),
            // NodeType::Bold => Ok(formatdoc! {"
            //     <b>
            //     {children}
            //     </b>
            // "}),
            // NodeType::StrikeThrough => Ok(formatdoc! {"
            //     <s>
            //     {children}
            //     </s>
            // "}),
            // NodeType::Code => Ok(formatdoc! {"
            //     <code>
            //     {children}
            //     </code>
            // "}),
        };

        Ok(html.into_string())
    }

    // Renders a list of children
    fn render_children(&self, children: &[Node]) -> Result<String, Error> {
        let mut children_str = vec![];
        for child in children.iter() {
            let child_str = self.render_node_iter(child)?;
            children_str.push(child_str);
        }
        Ok(children_str.join("\n"))
    }
}
