//! Tests

use crate::{HtmlNode, ParseOptions, RenderOptions};

static SAMPLE: &str = include_str!("tests/sample.html");

#[test]
fn test_parse_render() {
    let node = HtmlNode::parse(SAMPLE, ParseOptions::default()).unwrap();
    // eprintln!("{node:#?}");

    let html_str = node.render(RenderOptions::default()).unwrap();
    eprintln!("{html_str}");
}
