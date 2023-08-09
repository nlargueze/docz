//! Tests

use std::collections::HashMap;

use docz_ast::{AstParser, AstRenderer, Node, NodeType};

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
    let node = Node {
        ty: NodeType::Document {
            title: Some("hello".to_string()),
            authors: vec![],
        },
        attrs: HashMap::new(),
        value: None,
        children: vec![Node {
            ty: NodeType::Paragraph,
            attrs: HashMap::new(),
            value: None,
            children: vec![Node {
                ty: NodeType::Text,
                attrs: HashMap::new(),
                value: Some("Hello".to_string()),
                children: vec![],
            }],
        }],
    };

    let renderer = HTMLRenderer::new();
    let html = renderer.render(&node).unwrap();
    eprintln!("{html}");
}
