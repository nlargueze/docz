//! Unit tests

use docz_ast::{Attrs, Node, Parser, Renderer};

use super::*;

static SAMPLE: &str = include_str!("tests/sample.md");

#[test]
fn test_parse() {
    let parser = MdParser::new();
    let node = parser.parse(SAMPLE).unwrap();
    eprintln!("{node:#?}");
}

#[test]
#[ignore]
fn test_render() {
    let node = Node::Document {
        span: None,
        children: vec![],
        attrs: Attrs::new(),
        title: None,
        summary: None,
        authors: None,
    };
    let renderer = MdRenderer::new();
    let file_str = renderer.render_str(&node).unwrap();
    eprintln!("{file_str}");
}
