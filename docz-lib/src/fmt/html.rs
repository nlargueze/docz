//! HTML

use crate::{
    ast::Node,
    conv::{Exporter, Extractor},
    doc::Fragment,
};

use anyhow::{Error, Result};
use html_parser::Dom;

/// HTML extractor
#[derive(Debug)]
pub struct HtmlExtractor {}

impl Extractor for HtmlExtractor {
    fn extract(&self, data: &[u8]) -> Result<Fragment> {
        let data_str = std::str::from_utf8(data)?;
        let dom = Dom::parse(data_str)?;

        let errors = dom.errors;
        if !errors.is_empty() {
            let error = errors.first().unwrap().to_string();
            return Err(Error::msg(error));
        }

        let mut fragment = Fragment::default();

        for node in dom.children {
            match node {
                html_parser::Node::Text(txt) => {
                    let node = Node::text(&txt);
                    fragment.add_child(node);
                }
                html_parser::Node::Comment(comment) => {
                    let node = Node::comment(&comment);
                    fragment.add_child(node);
                }
                html_parser::Node::Element(elt) => {
                    elt.id;
                    elt.name;
                    elt.attributes;
                    for child in elt.children {
                        //
                    }
                }
            }
        }

        todo!()
    }
}

/// HTML exporter
#[derive(Debug)]
pub struct HtmlExporter {}

impl Exporter for HtmlExporter {
    fn export(
        &self,
        _doc: &crate::doc::Document,
    ) -> Result<std::collections::HashMap<std::path::PathBuf, Vec<u8>>> {
        todo!()
    }
}
