//! Debug renderer

use std::fs;

use crate::{cfg::Config, src::SourceData};

use super::Renderer;
use anyhow::Result;

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
    fn render(&self, cfg: &Config, data: &SourceData) -> Result<()> {
        let data_str = format!("{data:#?}");

        let build_dir = cfg.build_dir();
        let file_path = build_dir.join("debug.txt");
        fs::write(file_path, data_str)?;

        Ok(())
    }
}
