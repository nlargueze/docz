//! Service

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use docz_ast::{Attrs, DebugRenderer, Error, Node, Parser, Processor, Renderer};
use docz_ast_html::{HTMLParser, HTMLRenderer};
use docz_ast_md::{MdParser, MdRenderer};
use log::debug;

use crate::{
    cfg::Config,
    fmt::{FileExt, Format},
};

/// Doc service
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
        let debug_renderer = DebugRenderer::default();
        self.parser(Format::Markdown, md_parser)
            .parser(Format::Html, html_parser)
            .processor(FileAggregator::default())
            .renderer(Format::Markdown, md_renderer)
            .renderer(Format::Html, html_renderer)
            .renderer(Format::Debug, debug_renderer)
    }
}

impl Service {
    /// Builds the doc
    pub fn build(&self, format: Format) -> Result<()> {
        let nodes = self.parse()?;
        let nodes = self.process(nodes)?;
        let docs = self.render(format, &nodes)?;

        // save as files
        let build_dir = self.config.build_dir();
        if build_dir.exists() {
            fs::remove_dir_all(&build_dir)?;
        }
        fs::create_dir_all(&build_dir)?;

        for (i, doc) in docs.iter().enumerate() {
            let mut out_file = build_dir.join(if i == 0 {
                "doc".to_string()
            } else {
                format!("doc_{i}")
            });
            out_file.set_extension(format.to_string());
            fs::write(out_file, doc)?;
        }

        Ok(())
    }

    /// Parses the input files to AST nodes
    pub fn parse(&self) -> Result<Vec<Node>> {
        // get the source files
        let src_files = self.source_files()?;

        // parse each file to AST nodes
        let mut nodes = vec![];
        for src_file in &src_files {
            let node = self.parse_src_file(src_file)?;
            nodes.push(node);
        }

        Ok(nodes)
    }

    // Processes the nodes
    pub fn process(&self, nodes: Vec<Node>) -> Result<Vec<Node>> {
        let mut nodes = nodes;
        for processor in self.processors.iter() {
            nodes = processor.process(nodes)?;
        }
        Ok(nodes)
    }

    // Renders the nodes
    pub fn render(&self, format: Format, nodes: &[Node]) -> Result<Vec<String>> {
        let renderer = self
            .renderers
            .get(&format)
            .ok_or(anyhow!("No renderer for format {:?}", format))?;

        let mut nodes_str = vec![];
        for node in nodes {
            let node_str = renderer.render(node)?;
            nodes_str.push(node_str);
        }
        Ok(nodes_str)
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

    /// Parses a source file
    fn parse_src_file(&self, file: &Path) -> Result<Node> {
        debug!("Processing {:?}", file);

        let format = file.format().ok_or(anyhow!("Unsupported file format"))?;
        let parser = self
            .parsers
            .get(&format)
            .ok_or(anyhow!("No parser for format '{}'", format))?;

        let file_str = fs::read_to_string(file)?;
        Ok(parser.parse(&file_str)?)
    }
}

/// File aggregator
///
/// Each file is treated as a chapter and those chapters are aggregated into a single document
#[derive(Debug, Default)]
struct FileAggregator {}

impl Processor for FileAggregator {
    fn process(&self, nodes: Vec<Node>) -> Result<Vec<Node>, Error> {
        let chapters = nodes
            .into_iter()
            .filter_map(|node| {
                node.visit_and_modify(|node| match node {
                    Node::Document {
                        span,
                        children,
                        attrs,
                        ..
                    } => Some(Node::Chapter {
                        span: span.clone(),
                        children: children.clone(),
                        attrs: attrs.clone(),
                    }),
                    // any other node is kept as is
                    _ => Some(node.clone()),
                })
            })
            .collect::<Vec<_>>();

        Ok(vec![Node::Document {
            span: None,
            children: chapters,
            attrs: Attrs::default(),
            title: None,
            summary: None,
            authors: None,
        }])
    }
}
