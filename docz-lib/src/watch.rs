//! Watcher

use std::{thread, time::Duration};

use log::{debug, warn};
use notify::{Event, RecursiveMode, Watcher};

use crate::Service;

impl Service {
    /// Creates a watcher for the source directory
    pub fn watch_src<F>(&self, f: F) -> !
    where
        F: Fn(Event) + Send + Sync + 'static,
    {
        let mut watcher = notify::recommended_watcher(move |res| match res {
            Ok(event) => f(event),
            Err(e) => warn!("watch error: {:?}", e),
        })
        .unwrap();

        let src_dir = self.config.src_dir();
        debug!("watching: {}", src_dir.display());
        watcher.watch(&src_dir, RecursiveMode::Recursive).unwrap();

        loop {
            thread::sleep(Duration::from_secs(100_000_000_000));
        }
    }
}
