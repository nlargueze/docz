//! AST for PDF

mod ast;
mod error;
mod fonts;
mod render;

#[cfg(test)]
mod tests;

pub use ast::*;
pub use error::*;
pub use render::*;

/// PDF node
#[derive(Debug, Clone)]
pub enum PdfNode {
    Document {
        /// Paper size (width + height in mm)
        size: Option<(f64, f64)>,
        /// Title
        title: String,
        /// Authors
        authors: Option<Vec<String>>,
        /// Abstract
        summary: Option<String>,
        /// Children
        children: Vec<PdfNode>,
    },
    /// Section (chapter, subsection, etc.)
    Section {
        /// Section level
        level: usize,
        /// Index (eg. 1, 1.2, etc...)
        index: Option<String>,
        /// Title
        title: Option<String>,
        /// Children
        children: Vec<PdfNode>,
    },
    Paragraph {
        /// Children
        children: Vec<PdfNode>,
    },
    Text {
        value: String,
    },
    Image {
        url: String,
    },
    /// Latex formula
    Formula {
        inline: bool,
        value: String,
    },
}
