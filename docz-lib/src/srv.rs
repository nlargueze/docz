//! Service

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use docz_ast::{Attributes, Node, Parser, Processor, Renderer};
use docz_ast_html::{HTMLParser, HTMLRenderer};
use docz_ast_md::{MdParser, MdRenderer};
use log::debug;

use crate::{
    cfg::Config,
    fmt::{FileExt, Format},
};

/// Doc ervice
pub struct Service {
    /// Service configuration
    config: Config,
    /// Parsers
    parsers: HashMap<Format, Box<dyn Parser>>,
    /// Procesors
    processors: Vec<Box<dyn Processor>>,
    /// Renderers
    renderers: HashMap<Format, Box<dyn Renderer>>,
}

impl Service {
    /// Creates a new service
    pub fn new(config: Config) -> Self {
        Self {
            config,
            parsers: HashMap::new(),
            processors: vec![],
            renderers: HashMap::new(),
        }
    }

    /// Adds a parser
    pub fn parser(mut self, format: Format, parser: impl Parser + 'static) -> Self {
        self.parsers.insert(format, Box::new(parser));
        self
    }

    /// Adds a processor
    pub fn processor(mut self, processor: impl Processor + 'static) -> Self {
        self.processors.push(Box::new(processor));
        self
    }

    /// Adds a renderer
    pub fn renderer(mut self, format: Format, renderer: impl Renderer + 'static) -> Self {
        self.renderers.insert(format, Box::new(renderer));
        self
    }

    /// Adds a set of default parsers and renderers
    pub fn defaults(self) -> Self {
        let md_parser = MdParser::new();
        let md_renderer = MdRenderer::new();
        let html_parser = HTMLParser::new();
        let html_renderer = HTMLRenderer::new();
        self.parser(Format::Markdown, md_parser)
            .parser(Format::Html, html_parser)
            .renderer(Format::Markdown, md_renderer)
            .renderer(Format::Html, html_renderer)
    }
}

impl Service {
    /// Builds the doc
    pub fn build(&self, format: Format) -> Result<()> {
        let node = self.parse()?;
        let node = self.process(node)?;
        let out_str = self.render(format, &node)?;

        // save as files
        let build_dir = self.config.build_dir();
        let mut out_file = build_dir.join("doc");
        out_file.set_extension(format.to_string());
        fs::write(out_file, out_str)?;

        Ok(())
    }

    /// Parses a set of files to an AST node
    pub fn parse(&self) -> Result<Node> {
        // get the source files
        let src_files = self.source_files()?;

        // parse each file
        let mut children = vec![];
        for src_file in &src_files {
            let file_node = self.parse_file(src_file)?;
            debug!("file_node {:#?}", file_node);
            children.push(file_node);
        }

        Ok(Node::Document {
            position: None,
            children,
            attrs: Attributes::default(),
            title: Some(self.config.doc.title.clone()),
            summary: Some(self.config.doc.description.clone()),
            authors: Some(self.config.doc.authors.clone()),
        })
    }

    // Applies all the processors to a node
    pub fn process(&self, node: Node) -> Result<Node> {
        let mut node: Node = node;
        for processor in self.processors.iter() {
            node = processor.process(node)?;
        }
        Ok(node)
    }

    // Renders a node to the target format
    pub fn render(&self, format: Format, node: &Node) -> Result<String> {
        let renderer = self
            .renderers
            .get(&format)
            .ok_or(anyhow!("No renderer for format {:?}", format))?;
        let out_str = renderer.render(node)?;
        Ok(out_str)
    }

    /// Retrieves all the source files
    fn source_files(&self) -> Result<Vec<PathBuf>> {
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

    /// Parses a single source file
    fn parse_file(&self, file: &Path) -> Result<Node> {
        debug!("Processing {:?}", file);

        let format = file.format().ok_or(anyhow!("Unsupported file format"))?;
        let parser = self
            .parsers
            .get(&format)
            .ok_or(anyhow!("No parser for format {:?}", format))?;

        let file_str = fs::read_to_string(file)?;
        let node = parser.parse(&file_str)?;
        Ok(node)
    }
}
