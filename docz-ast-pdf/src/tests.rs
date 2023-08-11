//! Tests

use std::fs;

use docz_ast::{Attrs, Node, Renderer};

use crate::PDFRenderer;

#[test]
fn test_render() {
    let doc_ast = Node::Document {
        title: Some("My document".to_string()),
        summary: Some("My summary".to_string()),
        authors: Some(vec!["nick".to_string()]),
        attrs: Attrs::new(),
        span: None,
        children: vec![Node::Paragraph {
            attrs: Attrs::new(),
            span: None,
            children: vec![Node::Text {
                attrs: Attrs::new(),
                span: None,
                value: "Some text".to_string(),
            }],
        }],
    };

    let renderer = PDFRenderer::new();
    let bytes = renderer.render(&doc_ast).unwrap();
    fs::write("test.pdf", bytes).unwrap();
}
