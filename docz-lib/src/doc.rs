//! Document

use std::path::PathBuf;

/// Document
#[derive(Debug, Clone, Default)]
pub struct Document {
    /// Sections
    pub sections: Vec<Section>,
}

impl Document {
    /// Adds a section
    pub fn add_section(&mut self, section: Section) -> &mut Self {
        self.sections.push(section);
        self
    }
}

/// Document section
#[derive(Clone, Default)]
pub struct Section {
    /// Source file
    pub file: PathBuf,
    /// Content
    pub content: Vec<u8>,
    /// Children
    pub children: Vec<Section>,
}

impl std::fmt::Debug for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content_str =
            String::from_utf8(self.content.to_vec()).unwrap_or("_binary_".to_string());

        f.debug_struct("Section")
            .field("file", &self.file)
            .field("content", &content_str)
            .field("children", &self.children)
            .finish()
    }
}

impl Section {
    /// Creates a new [Section]
    pub fn new(src_file: impl Into<PathBuf>) -> Self {
        Self {
            file: src_file.into(),
            content: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Adds a subsection
    pub fn add_section(&mut self, section: Section) -> &mut Self {
        self.children.push(section);
        self
    }
}
