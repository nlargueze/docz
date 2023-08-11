//! Processors

use docz_ast::{Attrs, Error, Node, Processor};
use docz_ast_md::parse_frontmatter;
use log::error;
use serde::Deserialize;

/// Processor for chapter aggregation
///
/// Each file is treated as a chapter and those chapters are aggregated into a single document
#[derive(Debug, Default)]
pub struct ChapterAggregProcessor {}

impl Processor for ChapterAggregProcessor {
    fn process(&self, nodes: Vec<Node>) -> Result<Vec<Node>, Error> {
        let chapters = nodes
            .into_iter()
            .filter_map(|node| {
                node.visit_and_modify(&|node| match node {
                    Node::Document {
                        span,
                        children,
                        attrs,
                        ..
                    } => {
                        // look for metadata as direct children
                        #[derive(Deserialize)]
                        struct ChapterFrontMatter {
                            title: Option<String>,
                        }

                        let title = children
                            .iter()
                            .filter_map(|child| {
                                if let Node::Metadata { value, .. } = child {
                                    match parse_frontmatter::<ChapterFrontMatter>(value) {
                                        Ok(fm) => Some(fm.title),
                                        Err(err) => {
                                            error!("invalid frontmatter: {}", err);
                                            None
                                        }
                                    }
                                } else {
                                    None
                                }
                            })
                            .flatten()
                            .collect::<Vec<_>>()
                            .first()
                            .cloned();

                        Some(Node::Chapter {
                            span: span.clone(),
                            children: children.clone(),
                            attrs: attrs.clone(),
                            title,
                        })
                    }
                    // metadata is removed
                    Node::Metadata { .. } => None,
                    // any other node is kept as is
                    _ => Some(node.clone()),
                })
            })
            .collect::<Vec<_>>();

        Ok(vec![Node::Document {
            span: None,
            children: chapters,
            attrs: Attrs::default(),
            title: None,
            summary: None,
            authors: None,
        }])
    }
}

/// Processor to populate the document metadata
#[derive(Debug, Default)]
pub struct DocMetadataProcessor {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub authors: Option<Vec<String>>,
}

impl DocMetadataProcessor {
    /// Creates a new processor
    pub fn new(
        title: Option<String>,
        summary: Option<String>,
        authors: Option<Vec<String>>,
    ) -> Self {
        Self {
            title,
            summary,
            authors,
        }
    }
}

impl Processor for DocMetadataProcessor {
    fn process(&self, nodes: Vec<Node>) -> Result<Vec<Node>, Error> {
        Ok(nodes
            .into_iter()
            .filter_map(|node| {
                node.visit_and_modify(&|this_node| {
                    match this_node {
                        Node::Document {
                            title: _,
                            summary: _,
                            authors: _,
                            attrs,
                            span,
                            children,
                        } => {
                            // title = Some("ABC".to_string());
                            Some(Node::Document {
                                title: self.title.clone(),
                                summary: self.summary.clone(),
                                authors: self.authors.clone(),
                                attrs: attrs.clone(),
                                span: span.clone(),
                                children: children.clone(),
                            })
                        }
                        n => Some(n.clone()),
                    }
                })
            })
            .collect())
    }
}
