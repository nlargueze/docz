//! Rendering

mod dbg;
mod html;

pub use dbg::*;
pub use html::*;

use anyhow::Result;

use crate::{cfg::Config, src::SourceData};

/// Renderer
pub trait Renderer: Send {
    /// Registers the renderer (optional)
    fn register(&mut self, _cfg: &Config) -> Result<()> {
        Ok(())
    }

    /// Renders
    fn render(&self, cfg: &Config, data: &SourceData) -> Result<()>;
}
