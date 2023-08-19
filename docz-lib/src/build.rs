//! Build

use std::fs;

use anyhow::{anyhow, Result};
use log::{debug, trace};

use crate::{
    watch::{EventExt, WatchOptions, Watcher},
    Service,
};

impl Service {
    /// Builds the document
    pub fn build(&self) -> Result<()> {
        let src_tree = self.load_src_dir()?;

        // (re)create the build dir
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

    /// Removes the build folder
    pub fn remove_build_dir(&self) -> Result<()> {
        let build_dir = self.config.build_dir();
        if build_dir.exists() {
            fs::remove_dir_all(build_dir)?
        }
        Ok(())
    }

    /// Builds the document and watch for changes
    pub async fn build_and_watch(&mut self, opts: WatchOptions) -> Result<()> {
        self.build()?;

        let watched_dirs = self.watched_dirs();
        let mut watcher = Watcher::new(watched_dirs, Some(200))?;
        let mut rx_watch = watcher.start()?;
        loop {
            rx_watch.changed().await?;
            let event = rx_watch.borrow().clone();
            if event.triggers_rebuild() {
                debug!("Rebuilding ...");
                self.reload()?;
                self.build()?;
                debug!("Rebuilt OK");
                if let Some(on_rebuilt) = opts.on_rebuilt.as_ref() {
                    on_rebuilt(event.clone());
                }
            }
        }
    }
}
