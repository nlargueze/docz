//! Tests

use super::*;

static SAMPLE: &str = include_str!("tests/sample.html");

#[test]
fn parse_sample() {
    let conv = HtmlConverter::new();
    let node = conv.parse(SAMPLE).unwrap();
    eprintln!("{node:#?}");
}

// #[test]
// fn render_sample() {
//     let node = Node::default();
//     let conv = HtmlConverter::new();
//     let md = conv.render(&node).unwrap();
//     eprintln!("{md}");
// }
