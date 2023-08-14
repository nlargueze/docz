//! AST for PDF

mod rend;

#[cfg(test)]
mod tests;

use docz_ast::NodeData;
pub use rend::*;

/// PDF
#[derive(Debug, Clone)]
pub enum Pdf {
    Document {
        /// Paper size (size in mm)
        size: Option<(f64, f64)>,
        /// Title
        title: String,
        /// Authors
        authors: Option<Vec<String>>,
        /// Abstract
        summary: Option<String>,
    },
    Section {
        /// Section level
        level: usize,
        /// ToC index
        toc_index: Option<String>,
        /// Section title
        title: Option<String>,
    },
    Paragraph,
    Text {
        value: String,
    },
    Formula {
        value: String,
    },
    Image {
        //
    },
    Table {
        //
    },
    Reference {
        id: String,
        value: String,
    },
}

impl NodeData for Pdf {}
