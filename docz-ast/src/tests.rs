//! Tests

use crate::{Node, NodeData};

#[derive(Debug, Clone, PartialEq, Eq)]
enum TestData {
    Document,
    Chapter,
    Paragraph(String),
}

impl NodeData for TestData {}

#[test]
fn test_tree() {
    let node = Node::new(TestData::Document).with_child(
        // add chapter
        Node::new(TestData::Chapter).with_child(
            // add paragraph
            Node::new(TestData::Paragraph("Hello".to_string())),
        ),
    );
    assert_eq!(node.data, TestData::Document);
}

#[test]
fn test_visit() {
    let node = Node::new(TestData::Document).with_child(
        // add chapter
        Node::new(TestData::Chapter).with_child(
            // add paragraph
            Node::new(TestData::Paragraph("Hello".to_string())),
        ),
    );

    let nb_nodes = node.nb_nodes();
    assert_eq!(nb_nodes, 3);
    eprintln!("{node:#?}");
}
