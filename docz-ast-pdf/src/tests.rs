//! Tests

use std::fs;

use crate::{PdfNode, RenderOptions};

#[test]
fn test_render() {
    let pdf = PdfNode::Document {
        size: Some((210.0, 297.0)),
        title: "My document".to_string(),
        summary: Some("My summary".to_string()),
        authors: Some(vec!["nick".to_string()]),
        children: vec![PdfNode::Paragraph {
            children: vec![PdfNode::Text {
                value: "There are many variations of passages of Lorem Ipsum available, but the majority have suffered alteration in some form, by injected humour, or randomised words which don't look even slightly believable. If you are going to use a passage of Lorem Ipsum, you need to be sure there isn't anything embarrassing hidden in the middle of text. All the Lorem Ipsum generators on the Internet tend to repeat predefined chunks as necessary, making this the first true generator on the Internet. It uses a dictionary of over 200 Latin words, combined with a handful of model sentence structures, to generate Lorem Ipsum which looks reasonable. The generated Lorem Ipsum is therefore always free from repetition, injected humour, or non-characteristic words etc.".to_string(),
            }],
        }],
    };

    let pdf_bytes = pdf.render(RenderOptions::default()).unwrap();
    fs::write("test.pdf", pdf_bytes).unwrap();
}
