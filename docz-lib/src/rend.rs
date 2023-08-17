//! Rendering

mod dbg;
mod html;

pub use dbg::*;
pub use html::*;

use anyhow::Result;

use crate::{cfg::Config, doc::Document};

/// Renderer
pub trait Renderer: Send {
    /// Renders a document
    fn render(&self, cfg: &Config, doc: &Document) -> Result<()>;
}
