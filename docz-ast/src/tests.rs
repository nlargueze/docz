//! Tests

use crate::AstNode;

/// Test enum
#[derive(Debug, Clone)]
enum TestNode {
    Document { children: Vec<TestNode> },
    Paragraph { children: Vec<TestNode> },
    Text(String),
}

impl AstNode for TestNode {
    fn children(&self) -> Option<&Vec<Self>> {
        match self {
            TestNode::Document { children } => Some(children),
            TestNode::Paragraph { children } => Some(children),
            TestNode::Text(_) => None,
        }
    }

    fn children_mut(&mut self) -> Option<&mut Vec<Self>> {
        match self {
            TestNode::Document { children } => Some(children),
            TestNode::Paragraph { children } => Some(children),
            TestNode::Text(_) => None,
        }
    }
}

#[test]
fn test_visit() {
    let node = TestNode::Document {
        children: vec![TestNode::Paragraph {
            children: vec![TestNode::Text("Hello".to_string())],
        }],
    };
    eprintln!("{node:#?}");

    let mut n = 0;
    node.visit(&mut |_| n += 1);
    assert_eq!(n, 3);
}
