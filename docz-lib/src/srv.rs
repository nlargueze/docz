//! Service

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use log::debug;

use crate::{
    cfg::Config,
    conv::{Exporter, Extractor, Processor},
    doc::{Document, Fragment},
    fmt::{mkd::MarkdownExtractor, FileExt, Format},
};

/// Service
pub struct Service {
    /// Service configuration
    config: Config,
    /// Extractors
    extractors: HashMap<Format, Box<dyn Extractor>>,
    /// Processors
    processors: HashMap<String, Box<dyn Processor>>,
    /// Exporters
    exporters: HashMap<Format, Box<dyn Exporter>>,
}

impl Service {
    /// Creates a new service
    pub fn new(config: Config) -> Self {
        Self {
            config,
            extractors: HashMap::new(),
            processors: HashMap::new(),
            exporters: HashMap::new(),
        }
    }

    /// Adds a set of defaults
    pub fn defaults(self) -> Self {
        self.extractor(Format::Markdown, MarkdownExtractor::default())
    }

    /// Adds an extractor to the service
    pub fn extractor(mut self, format: Format, extractor: impl Extractor + 'static) -> Self {
        self.extractors.insert(format, Box::new(extractor));
        self
    }

    /// Adds a processor to the service
    pub fn processor(mut self, key: &str, processor: impl Processor + 'static) -> Self {
        self.processors.insert(key.to_string(), Box::new(processor));
        self
    }

    /// Adds an exporter to the service
    pub fn exporter(mut self, format: Format, exporter: impl Exporter + 'static) -> Self {
        self.exporters.insert(format, Box::new(exporter));
        self
    }
}

impl Service {
    /// Extracts a document from the source files
    pub fn extract(&self) -> Result<Document> {
        // init document
        let mut doc = Document::default();

        // get the files
        let files = self.get_files()?;

        // process each file
        for file in &files {
            let fragment = self.process_file(file)?;
            debug!("Fragment {:#?}", fragment);
            doc.add_fragment(fragment);
        }

        Ok(doc)
    }

    /// Exports a document
    pub fn export(&self, format: Format, doc: &Document) -> Result<()> {
        let exporter = self.exporters.get(&format).ok_or(anyhow!("no exporter"))?;

        let files = exporter.export(doc)?;

        let build_dir = self.config.build_dir();
        for (file, data) in files {
            let path = build_dir.join(file);
            fs::write(path, data)?;
        }

        Ok(())
    }

    /// Retrieves all the files
    fn get_files(&self) -> Result<Vec<PathBuf>> {
        let src_dir = self.config.src_dir();
        let mut files = vec![];
        for file in &self.config.doc.files {
            let file = src_dir.join(file);
            if !file.exists() {
                return Err(anyhow!("file {:?} does not exist", file));
            }
            files.push(file);
        }
        Ok(files)
    }

    /// Processes a single file
    pub fn process_file(&self, file: &Path) -> Result<Fragment> {
        debug!("Processing {:?}", file);

        let format = match file.format() {
            Some(f) => f,
            None => {
                debug!("EXT {:#?}", file.extension());
                return Err(anyhow!("Unsupported file format"));
            }
        };

        let extractor = match self.extractors.get(&format) {
            Some(e) => e,
            None => {
                return Err(anyhow!("No extractor for format {:?}", format));
            }
        };

        let data = fs::read_to_string(file)?;
        extractor.extract(data.as_bytes())
    }
}
