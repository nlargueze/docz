//! Build

use std::fs;

use anyhow::{anyhow, Result};
use log::{debug, trace};
use notify::Event;

use crate::{
    watch::{EventExt, Watcher},
    Service,
};

/// Build options
#[derive(Default)]
pub struct BuildOptions {
    /// Watch mode
    pub watch: bool,
    /// On rebuilt
    pub on_rebuilt: Option<Box<dyn Fn(Event) + Send + Sync>>,
}

impl Service {
    /// Removes the build folder
    pub fn remove_build_dir(&self) -> Result<()> {
        let build_dir = self.config.build_dir();
        if build_dir.exists() {
            fs::remove_dir_all(build_dir)?
        }
        Ok(())
    }

    /// Builds the documentation
    pub async fn build(&self, opts: BuildOptions) -> Result<()> {
        self.build_once()?;

        if opts.watch {
            let watched_dirs = self.watched_dirs();
            let mut watcher = Watcher::new(watched_dirs, Some(200))?;
            let mut rx_watch = watcher.start()?;
            loop {
                rx_watch.changed().await?;
                let event = rx_watch.borrow().clone();
                if event.triggers_rebuild() {
                    debug!("Rebuilding ...");
                    self.build_once()?;
                    debug!("Rebuilt OK");
                    if let Some(on_rebuilt) = opts.on_rebuilt.as_ref() {
                        on_rebuilt(event.clone());
                    }
                }
            }
        }

        Ok(())
    }

    /// Builds the documentation
    pub(crate) fn build_once(&self) -> Result<()> {
        let src_tree = self.load_src_dir()?;

        // recreate the build dir
        let build_dir = self.config.build_dir();
        self.remove_build_dir()?;
        fs::create_dir(&build_dir)?;

        for id in self.config.output_ids() {
            if let Some(renderer) = self.renderers.get(id) {
                trace!("Rendering output ({id})");
                renderer.render(&self.config, &src_tree)?;
            } else {
                return Err(anyhow!(
                    "Invalid output type ({}). Check the config file or add a renderer",
                    id
                ));
            };
        }

        Ok(())
    }
}
