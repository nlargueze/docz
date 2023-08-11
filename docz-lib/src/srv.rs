//! Service

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use docz_ast::{DebugRenderer, Node, Parser, Processor, Renderer};
use docz_ast_html::{HTMLParser, HTMLRenderer};
use docz_ast_md::{MdParser, MdRenderer};
use fs_extra::dir::CopyOptions;
use log::debug;

use crate::{
    cfg::Config,
    fmt::{FileExt, Format},
    ChapterAggregProcessor, DocMetadataProcessor,
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
        let html_parser = HTMLParser::new();

        let chapter_processor = ChapterAggregProcessor::default();
        let doc_meta_processor = DocMetadataProcessor::new(
            Some(self.config.doc.title.clone()),
            Some(self.config.doc.description.clone()),
            Some(self.config.doc.authors.clone()),
        );

        let html_renderer = HTMLRenderer::new();
        let md_renderer = MdRenderer::new();
        let debug_renderer = DebugRenderer::default();

        self.parser(Format::Markdown, md_parser)
            .parser(Format::Html, html_parser)
            .processor(chapter_processor)
            .processor(doc_meta_processor)
            .renderer(Format::Markdown, md_renderer)
            .renderer(Format::Html, html_renderer)
            .renderer(Format::Debug, debug_renderer)
    }
}

impl Service {
    /// Builds the doc
    pub fn build(&self, format: Format) -> Result<()> {
        debug!("build config \n {:#?}", self.config);
        let nodes = self.parse()?;
        let nodes = self.process(nodes)?;
        let docs = self.render(format, &nodes)?;

        // reset build dir
        let build_dir = self.config.build_dir();
        if build_dir.exists() {
            fs::remove_dir_all(&build_dir)?;
        }
        fs::create_dir_all(&build_dir)?;

        // copy static files
        self.copy_assets_to_build_dir()?;

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
        // parse each file to AST nodes
        let mut nodes = vec![];
        for src_file in &self.source_files()? {
            debug!("parsing file ... \n {}", src_file.to_string_lossy());
            let node = self.parse_file(src_file)?;
            debug!("AST node \n {:#?}", node);
            nodes.push(node);
        }

        Ok(nodes)
    }

    // Processes the nodes
    pub fn process(&self, nodes: Vec<Node>) -> Result<Vec<Node>> {
        let mut nodes = nodes;
        for processor in self.processors.iter() {
            nodes = processor.process(nodes)?;
            debug!("Post process \n {:#?}", nodes);
        }
        Ok(nodes)
    }

    // Renders the nodes to a specific format
    pub fn render(&self, format: Format, nodes: &[Node]) -> Result<Vec<Vec<u8>>> {
        let renderer = self
            .renderers
            .get(&format)
            .ok_or(anyhow!("No renderer for format {:?}", format))?;

        let mut nodes_str = vec![];
        for node in nodes {
            let node_bytes = renderer.render(node)?;
            if format.is_binary() {
                debug!("Post render ({format})\n BINARY");
            } else {
                let node_str = renderer.render_str(node)?;
                debug!("Post render ({format})\n {node_str}");
            }

            nodes_str.push(node_bytes);
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
    fn parse_file(&self, file: &Path) -> Result<Node> {
        let format = file.format().ok_or(anyhow!("Unsupported file format"))?;
        let parser = self
            .parsers
            .get(&format)
            .ok_or(anyhow!("No parser for format '{}'", format))?;

        let file_str = fs::read_to_string(file)?;
        Ok(parser.parse(&file_str)?)
    }

    /// Copies the assets dir to the build dir
    fn copy_assets_to_build_dir(&self) -> Result<()> {
        let assets_dir = self.config.assets_dir();
        if assets_dir.exists() {
            let build_dir = self.config.build_dir();
            fs_extra::dir::copy(assets_dir, build_dir, &CopyOptions::new())?;
        }
        Ok(())
    }
}
