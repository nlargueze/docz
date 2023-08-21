//! Rendering

mod dbg;
#[cfg(feature = "epub")]
mod epub;
mod html;

pub use dbg::*;
#[cfg(feature = "epub")]
pub use epub::*;
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
