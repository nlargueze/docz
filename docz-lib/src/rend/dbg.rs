//! Debug renderer

use std::fs;

use crate::cfg::Config;

use super::Renderer;
use anyhow::Result;
use log::debug;

/// Renderer for debugging
#[derive(Debug, Default)]
pub struct DebugRenderer {}

impl DebugRenderer {
    /// Creates a new debug renderer
    pub fn new() -> Self {
        Self::default()
    }
}

impl Renderer for DebugRenderer {
    fn id(&self) -> &'static str {
        "debug"
    }

    fn render(&self, cfg: &Config, doc: &crate::doc::Document) -> Result<()> {
        debug!("Renderer (debug)");
        let doc_str = format!("{doc:#?}");

        let build_dir = cfg.build_dir();
        let file_path = build_dir.join("debug.txt");
        fs::write(file_path, doc_str)?;
        Ok(())
    }
}
