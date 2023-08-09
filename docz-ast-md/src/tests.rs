//! Unit tests

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
    let node = Node::default();
    let renderer = MdRenderer::new();
    let file_str = renderer.render(&node).unwrap();
    eprintln!("{file_str}");
}
