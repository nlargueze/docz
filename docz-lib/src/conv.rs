//! Converter

use std::{collections::HashMap, path::PathBuf};

use crate::doc::{Document, Fragment};

use anyhow::Result;

/// An extractor is responsible for parsing a file into a [Fragment]
pub trait Extractor {
    /// Extracts bytes to a [Fragment]
    fn extract(&self, data: &[u8]) -> Result<Fragment>;
}

/// A processor processes a fragment
pub trait Processor {
    /// Processes a fragment into another fragment
    fn process(&self, fragment: Fragment) -> Result<Fragment>;
}

/// Exports a document to its target format
///
/// It returns a map of files and their content
pub trait Exporter {
    /// Exports a document to its target format
    fn export(&self, doc: &Document) -> Result<HashMap<PathBuf, Vec<u8>>>;
}
