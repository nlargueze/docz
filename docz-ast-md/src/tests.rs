//! Unit tests

// use docz_ast::{Parser, Renderer};

// use super::*;

use crate::{MdNode, ParseOptions};

static SAMPLE_FILE: &str = include_str!("tests/sample.md");

#[test]
fn test_parse_render() {
    let node = MdNode::parse(SAMPLE_FILE, ParseOptions::default()).unwrap();
    eprintln!("{node:#?}");

    let file_str = node.render(crate::RenderOptions::default()).unwrap();
    eprintln!("{file_str}");
}
