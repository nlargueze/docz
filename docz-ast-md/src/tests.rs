//! Unit tests

use super::*;

static SAMPLE: &str = include_str!("tests/sample.md");

#[test]
fn parse_sample() {
    let conv = MdConverter::new();
    let node = conv.parse(SAMPLE).unwrap();
    eprintln!("{node:#?}");
}

#[test]
fn render_sample() {
    let node = Node::default();
    let conv = MdConverter::new();
    let md = conv.render(&node).unwrap();
    eprintln!("{md}");
}
