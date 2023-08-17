//! Rendering

mod dbg;
mod html;

pub use dbg::*;
pub use html::*;

use anyhow::Result;

use crate::{cfg::Config, doc::Document};

/// Renderer
pub trait Renderer: Send {
    /// Returns the renderer ID
    fn id(&self) -> &'static str;

    /// Renders a document
    fn render(&self, cfg: &Config, doc: &Document) -> Result<()>;
}
