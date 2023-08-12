//! Renderer

use docz_ast::{Error, Node, Renderer};
use maud::{html, PreEscaped, DOCTYPE};

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
    fn is_binary(&self) -> bool {
        false
    }

    fn render(&self, node: &Node) -> Result<Vec<u8>, Error> {
        Ok(self.render_node_iter(node)?.into_string().into_bytes())
    }
}

impl HTMLRenderer {
    // Renders a node recursively
    #[allow(clippy::only_used_in_recursion)]
    fn render_node_iter(&self, node: &Node) -> Result<PreEscaped<String>, Error> {
        let html = match node {
            Node::Document {
                span: _,
                children,
                attrs: _,
                title,
                summary: _,
                authors,
            } => html! {
                @let title = title.clone().unwrap_or("Title".to_string());
                @let children = self.render_children(children)?;

                (DOCTYPE)
                html xmlns="http://www.w3.org/1999/xhtml" lang="en" {
                    head {
                        title { (title) }
                        meta charset="UTF-8";
                        @if let Some(authors) = authors {
                            @for author in authors {
                                meta name="author" content=(author);
                            }
                        }
                    }
                    body {
                        (children)
                    }
                }
            },
            Node::Fragment {
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                (children)
            },
            Node::Chapter {
                title: _,
                span: _,
                attrs: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                div data-node="chapter" {
                    (children)
                }
            },
            Node::Section {
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                div data-node="section" {
                    (children)
                }
            },
            Node::Heading {
                level,
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;
                @match level {
                    1 => h1 {
                        (children)
                    },
                    2 => h2 {
                        (children)
                    },
                    3 => h3 {
                        (children)
                    },
                    4 => h4 {
                        (children)
                    },
                    5 => h5 {
                        (children)
                    },
                    6 => h6 {
                        (children)
                    },
                    _ => h1 {
                        (children)
                    },
                }
            },
            Node::BlockQuote {
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                blockquote {
                    (children)
                }
            },
            Node::LineBreak { span: _ } => html! {
                br;
            },
            Node::SoftBreak { span: _ } => html! {
                // TODO: add soft break here
            },
            Node::CodeBlock {
                info,
                attrs: _,
                span: _,
                value,
            } => html! {
                pre data-info=(info) {
                    code {
                        (value)
                    }
                }
            },
            Node::Definition {
                id: _,
                label: _,
                url: _,
                title: _,
                attrs: _,
                span: _,
                children: _,
            } => html! {
                // TODO: add definition here
            },
            Node::Italic {
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                i {
                    (children)
                }
            },
            Node::Html {
                attrs: _,
                span: _,
                value,
            } => html! {
                (value)
            },
            Node::Image {
                url,
                alt,
                title: _,
                attrs: _,
                span: _,
            } => html! {
                img src=(url) alt=(alt);
            },
            Node::ImageRef {
                id: _,
                label: _,
                alt: _,
                attrs: _,
                span: _,
            } => html! {
                // TODO: image ref
            },
            Node::InlineCode {
                attrs: _,
                span: _,
                value,
            } => html! {
                code {
                    (value)
                }
            },
            Node::Link {
                url,
                title,
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                a href=(url) title=(title) {
                    (children)
                }
            },
            Node::LinkRef {
                id: _,
                label: _,
                attrs: _,
                span: _,
                children: _,
            } => html! {
                // TODO: link ref
            },
            Node::List {
                ordered,
                start: _,
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                @if *ordered {
                    ol {
                        (children)
                    }
                } @else {
                    ul {
                        (children)
                    }
                }
            },
            Node::ListItem {
                checked,
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                @if let Some(checked) = *checked {
                    @if checked {
                        li data-checked="true" {
                            (children)
                        }
                    } @else {
                        li {
                            (children)
                        }
                    }
                } else {
                    li {
                        (children)
                    }
                }

            },
            Node::Paragraph {
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                p {
                    (children)
                }
            },
            Node::Bold {
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                b {
                    (children)
                }
            },
            Node::Superscript {
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                sup {
                    (children)
                }
            },
            Node::Text {
                attrs: _,
                span: _,
                value,
            } => html! {
                (value)
            },
            Node::ThematicBreak { attrs: _, span: _ } => html! {
                hr;
            },
            Node::StrikeThrough {
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                s {
                    (children)
                }
            },
            Node::FootnoteDef {
                id,
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                div data-type="footnote" data-id=(id) {
                    (children)
                }
            },
            Node::FootnoteRef {
                id,
                attrs: _,
                span: _,
            } => html! {
                span {
                    a href=(format!("#{}", id)) data-type="footnote-ref" {
                        (id)
                    }
                }
            },
            Node::Table {
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                table {
                    (children)
                }
            },
            Node::TableRow {
                is_header,
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                @if *is_header {
                    thead {
                        tr {
                            (children)
                        }
                    }
                } @else {
                    tbody {
                        tr {
                            (children)
                        }
                    }
                }

            },
            Node::TableCell {
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                td {
                    (children)
                }
            },
            Node::FrontMatter { .. } => html! {},
            Node::DescrList {
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                dl {
                    (children)
                }
            },
            Node::DescrItem {
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                dt {
                    (children)
                }
            },
            Node::DescrTerm {
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                dt {
                    (children)
                }
            },
            Node::DescrDetail {
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                dd {
                    (children)
                }
            },
            Node::Comment { .. } => html! {},
            Node::Other {
                name,
                attrs: _,
                span: _,
                children,
            } => html! {
                @let children = self.render_children(children)?;

                div data-name=(name) {
                    (children)
                }
            },
        };

        Ok(html)
    }

    // Renders a list of children
    fn render_children(&self, children: &[Node]) -> Result<PreEscaped<String>, Error> {
        let mut children_str = vec![];
        for child in children.iter() {
            let child_str = self.render_node_iter(child)?;
            children_str.push(child_str);
        }
        Ok(html! {
            @for child in children_str {
                (child)
            }
        })
    }
}
