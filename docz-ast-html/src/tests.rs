//! Tests

use docz_ast::{Attributes, Node, Parser, Renderer};

use super::*;

static SAMPLE: &str = include_str!("tests/sample.html");

#[test]
#[ignore]
fn test_parse() {
    let parser = HTMLParser::new();
    let node = parser.parse(SAMPLE).unwrap();
    eprintln!("{node:#?}");
}

#[test]
fn test_render() {
    let node = Node::Document {
        position: None,
        attrs: Attributes::default(),
        title: Some("Title".to_string()),
        summary: None,
        authors: None,
        children: vec![Node::Paragraph {
            position: None,
            attrs: Attributes::default(),
            children: vec![Node::Text {
                position: None,
                attrs: Attributes::default(),
                value: "Hello".to_string(),
            }],
        }],
    };

    let renderer = HTMLRenderer::new();
    let html = renderer.render(&node).unwrap();
    eprintln!("{html}");
}
