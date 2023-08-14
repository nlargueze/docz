//! Tests

use docz_ast::{Parser, Renderer};

use super::*;

static SAMPLE: &str = include_str!("tests/sample.html");

#[test]
fn test_parse_render() {
    let parser = HTMLParser::new();
    let node = parser.parse(SAMPLE.as_bytes()).unwrap();
    // eprintln!("{node:#?}");

    let renderer = HTMLRenderer::new();
    let html = renderer.render(&node).unwrap();
    let html_str = String::from_utf8(html).unwrap();
    eprintln!("{html_str}");
}
