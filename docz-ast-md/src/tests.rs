//! Unit tests

use docz_ast::{Parser, Renderer};

use super::*;

static SAMPLE_FILE: &str = include_str!("tests/sample.md");

#[test]
fn test_parse_render() {
    let parser = MdParser::new();
    let node = parser.parse(SAMPLE_FILE.as_bytes()).unwrap();
    // eprintln!("{}", node.to_json(true).unwrap());

    let renderer = MdRenderer::new();
    let data = renderer.render(&node).unwrap();
    let data_str = String::from_utf8(data).unwrap();
    eprintln!("{data_str}");
}
