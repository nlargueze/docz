//! Unit tests

use docz_ast::{Parser, Renderer};

use super::*;

static SAMPLE_FILE: &str = include_str!("tests/sample.md");

#[test]
fn test_parse_render() {
    let parser = MdParser::new();
    let node = parser.parse(SAMPLE_FILE).unwrap();
    // eprintln!("{}", node.to_json(true).unwrap());

    let renderer = MdRenderer::new();
    let file_str = renderer.render_str(&node).unwrap();
    eprintln!("{file_str}");
}
