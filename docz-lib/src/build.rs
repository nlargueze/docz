//! Build

use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use log::{debug, warn};
use notify::{EventKind, RecursiveMode, Watcher};
use tokio::sync::watch;

use crate::Service;

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
    pub fn build(&self) -> Result<()> {
        let src_tree = self.load_src_dir()?;

        // recreate the build dir
        let build_dir = self.config.build_dir();
        self.remove_build_dir()?;
        fs::create_dir(&build_dir)?;

        for id in self.config.output_ids() {
            if let Some(renderer) = self.renderers.get(id) {
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

    /// Builds the documentation
    pub async fn build_with_watch<F>(&self, on_rebuilt: F) -> Result<()>
    where
        F: Fn(Vec<PathBuf>),
    {
        // setup watch
        let (tx_watch, mut rx_watch) = watch::channel(vec![]);
        let mut watcher = notify::recommended_watcher(
            move |res: Result<notify::Event, notify::Error>| match res {
                Ok(event) => {
                    // debug!("Received event: {:?}", event);
                    let rebuild = matches!(
                        event.kind,
                        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
                    );
                    if rebuild {
                        if let Err(err) = tx_watch.send(event.paths) {
                            warn!("Error sending watch event: {}", err)
                        }
                    }
                }
                Err(e) => warn!("watch error: {:?}", e),
            },
        )?;

        let src_dir = self.config.src_dir();
        debug!("Watching {:?}", src_dir);
        watcher.watch(&src_dir, RecursiveMode::Recursive)?;

        self.build()?;
        loop {
            rx_watch.changed().await?;
            let paths = rx_watch.borrow().to_vec();
            debug!("Rebuilding ...");
            self.build()?;
            debug!("Rebuilt OK");
            on_rebuilt(paths);
        }
    }
}
