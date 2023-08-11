//! Tests

use std::fs;

use docz_ast::{Attrs, Node, Renderer};

use crate::EPubRenderer;

#[test]
fn test_epub() {
    let node = Node::Document {
        title: Some("My document".to_string()),
        summary: None,
        authors: None,
        attrs: Attrs::new(),
        span: None,
        children: vec![Node::Chapter {
            title: None,
            span: None,
            attrs: Attrs::new(),
            children: vec![Node::Text {
                attrs: Attrs::new(),
                span: None,
                value: " Some text".to_string(),
            }],
        }],
    };

    let renderer = EPubRenderer::new("My title")
        .stylesheet("body { background-color: #eee; }")
        .author("John Doe")
        .cover("cover.png", "image/png", vec![0; 100]);
    let bytes = renderer.render(&node).unwrap();
    fs::write("test.epub", bytes).unwrap();
}
